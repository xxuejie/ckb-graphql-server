mod input_types;
mod types;

use ckb_db::{iter::DBIter, Col};
use ckb_jsonrpc_types::{BlockNumber, Byte32};
use ckb_store::{ChainStore, StoreCache, COLUMNS};
use ckb_types::prelude::*;
use clap::{App, Arg};
use futures::future;
use hyper::{
    rt::{self, Future},
    service::service_fn,
    Body, Method, Response, Server, StatusCode,
};
use juniper::{EmptyMutation, FieldResult, RootNode};
use rocksdb::{
    ops::{GetColumnFamilys, GetPinnedCF, IterateCF, OpenCF},
    DBPinnableSlice, IteratorMode, Options, ReadOnlyDB,
};
use serde_json::from_str as from_json_str;
use serde_plain::from_str;
use std::convert::TryFrom;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct Context {
    db: ReadOnlyDB,
}

impl<'a> ChainStore<'a> for Context {
    type Vector = DBPinnableSlice<'a>;

    fn cache(&'a self) -> Option<&'a StoreCache> {
        None
    }

    fn get(&'a self, col: Col, key: &[u8]) -> Option<Self::Vector> {
        let cf = self.db.cf_handle(col).expect("db cf_handle");
        self.db.get_pinned_cf(cf, &key).expect("db get")
    }

    fn get_iter<'i>(&'i self, col: Col, mode: IteratorMode) -> DBIter {
        let cf = self.db.cf_handle(col).expect("db cf_handle");
        self.db.iterator_cf(cf, mode).expect("db iter")
    }
}

impl juniper::Context for Context {}

struct Query;

#[juniper::object(
    Context = Context,
)]

impl Query {
    fn apiVersion() -> &str {
        // TODO: read from CKB version
        "0.1.0"
    }

    fn tip_header(context: &Context) -> FieldResult<Option<types::Header>> {
        match context.get_tip_header() {
            Some(header_view) => Ok(Some(types::Header(header_view.into()))),
            None => Ok(None),
        }
    }

    fn tip_block(context: &Context) -> FieldResult<Option<types::Block>> {
        Ok(context
            .get_tip_header()
            .and_then(|header| context.get_block(&header.hash()))
            .map(|b| types::Block(b.into())))
    }

    fn get_transaction(context: &Context, hash: String) -> FieldResult<Option<types::Transaction>> {
        let hash: Byte32 = from_str(&hash)?;
        match context.get_transaction(&hash.into()) {
            Some((transaction_view, _)) => Ok(Some(types::Transaction(transaction_view.into()))),
            None => Ok(None),
        }
    }

    fn get_block(
        context: &Context,
        number: Option<String>,
        hash: Option<String>,
    ) -> FieldResult<Option<types::Block>> {
        let block = match (number, hash) {
            (Some(number), None) => {
                // serde_plain has problems deserializing BlockNumber
                let number: BlockNumber = from_json_str(&format!("\"{}\"", number))?;
                context
                    .get_block_hash(number.into())
                    .and_then(|hash| context.get_block(&hash))
            }
            (None, Some(hash)) => {
                let hash: Byte32 = from_str(&hash)?;
                context.get_block(&hash.into())
            }
            (Some(number), Some(hash)) => {
                // serde_plain has problems deserializing BlockNumber
                let number: BlockNumber = from_json_str(&format!("\"{}\"", number))?;
                let target_hash: Byte32 = from_str(&hash)?;
                let block = context
                    .get_block_hash(number.into())
                    .and_then(|hash| context.get_block(&hash));
                if let Some(block) = &block {
                    if block.hash() != target_hash.into() {
                        return Err("Block at specified number does not match block hash!".into());
                    };
                };
                block
            }
            (None, None) => {
                return Err("You must specify either block number or block hash!".into())
            }
        };
        Ok(block.map(|b| types::Block(b.into())))
    }

    fn get_cells(
        context: &Context,
        out_points: Vec<input_types::InputOutPoint>,
        skip_missing: Option<bool>,
    ) -> FieldResult<Vec<types::OutPoint>> {
        out_points
            .iter()
            .try_fold(vec![], |mut results, out_point| {
                let rpc_out_point = ckb_jsonrpc_types::OutPoint::try_from(out_point)?;
                let tx_hash = rpc_out_point.tx_hash.pack();
                let index: u32 = rpc_out_point.index.into();
                let exists = context
                    .get_tx_meta(&tx_hash)
                    .map(|tx_meta| {
                        if (index as usize) < tx_meta.len() {
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);
                match (exists, skip_missing.unwrap_or(false)) {
                    (true, _) => {
                        results.push(rpc_out_point);
                        Ok(results)
                    }
                    (false, true) => Ok(results),
                    (false, false) => Err(format!("Cannot find cell: 0x{:x}@{}!", tx_hash, index)
                        .to_string()
                        .into()),
                }
            })
            .map(|ops| ops.into_iter().map(types::OutPoint).collect())
    }
}

fn main() {
    env_logger::init();

    let matches = App::new("CKB GraphQL server")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .help("Listened address")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db")
                .short("d")
                .long("db")
                .help("CKB's rocksdb path")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let db_path = matches.value_of("db").unwrap().to_string();
    let cfnames: Vec<_> = (0..COLUMNS).map(|c| c.to_string()).collect();
    let cf_options: Vec<String> = cfnames.into_iter().map(|n| n).collect();
    // Test opening DB first before initializing server.
    ReadOnlyDB::open_cf(&Options::default(), &db_path, &cf_options).expect("rocksdb");

    let root_node = Arc::new(RootNode::new(Query, EmptyMutation::<Context>::new()));
    let ctx = Arc::new(RwLock::new(Arc::new(Context {
        db: ReadOnlyDB::open_cf(&Options::default(), &db_path, &cf_options).expect("rocksdb"),
    })));
    let ctx2 = Arc::clone(&ctx);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        let mut c = ctx.write().unwrap();
        *c = Arc::new(Context {
            db: ReadOnlyDB::open_cf(&Options::default(), &db_path, &cf_options).expect("rocksdb"),
        });
    });
    let new_service = move || {
        let root_node = root_node.clone();
        let ctx2 = ctx2.clone();
        service_fn(move |req| -> Box<dyn Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let ctx = ctx2.read().unwrap().clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Box::new(juniper_hyper::graphiql("/graphql")),
                (&Method::GET, "/graphql") => Box::new(juniper_hyper::graphql(root_node, ctx, req)),
                (&Method::POST, "/graphql") => {
                    Box::new(juniper_hyper::graphql(root_node, ctx, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let addr = matches
        .value_of("listen")
        .unwrap()
        .parse()
        .expect("parse listen address");
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}
