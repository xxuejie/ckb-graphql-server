use crate::Context;
use ckb_jsonrpc_types;
use ckb_store::ChainStore;
use ckb_types::{self, packed, prelude::*, H256};
use serde_plain::to_string;

pub struct Bytes(pub ckb_jsonrpc_types::JsonBytes);

#[juniper::object]
impl Bytes {
    fn hash(&self) -> String {
        let hash = packed::CellOutput::calc_data_hash(self.0.as_bytes());
        let hash: H256 = hash.unpack();
        to_string(&hash).expect("serde")
    }

    fn length(&self) -> String {
        let length: ckb_jsonrpc_types::Uint64 = (self.0.len() as u64).into();
        to_string(&length).expect("serde")
    }

    fn content(&self) -> String {
        to_string(&self.0).expect("serde")
    }
}

pub struct Header(pub ckb_jsonrpc_types::HeaderView);

#[juniper::object]
impl Header {
    fn version(&self) -> String {
        to_string(&self.0.inner.version).expect("serde")
    }

    fn compact_target(&self) -> String {
        to_string(&self.0.inner.compact_target).expect("serde")
    }

    fn parent_hash(&self) -> String {
        to_string(&self.0.inner.parent_hash).expect("serde")
    }

    fn timestamp(&self) -> String {
        to_string(&self.0.inner.timestamp).expect("serde")
    }

    fn number(&self) -> String {
        to_string(&self.0.inner.number).expect("serde")
    }

    fn epoch(&self) -> String {
        to_string(&self.0.inner.epoch).expect("serde")
    }

    fn transactions_root(&self) -> String {
        to_string(&self.0.inner.transactions_root).expect("serde")
    }

    fn proposals_hash(&self) -> String {
        to_string(&self.0.inner.proposals_hash).expect("serde")
    }

    fn uncles_hash(&self) -> String {
        to_string(&self.0.inner.uncles_hash).expect("serde")
    }

    fn dao(&self) -> String {
        to_string(&self.0.inner.dao).expect("serde")
    }

    fn nonce(&self) -> String {
        to_string(&self.0.inner.nonce).expect("serde")
    }

    fn hash(&self) -> String {
        to_string(&self.0.hash).expect("serde")
    }
}

pub struct Transaction(pub ckb_jsonrpc_types::TransactionView);

#[juniper::object(
    Context = Context,
)]
impl Transaction {
    fn version(&self) -> String {
        to_string(&self.0.inner.version).expect("serde")
    }

    fn cell_deps(&self) -> Vec<CellDep> {
        self.0
            .inner
            .cell_deps
            .iter()
            .map(|dep| CellDep(dep))
            .collect()
    }

    fn header_deps(&self) -> Vec<String> {
        self.0
            .inner
            .header_deps
            .iter()
            .map(|dep| to_string(&dep).expect("serde"))
            .collect()
    }

    fn inputs(&self) -> Vec<CellInput> {
        self.0
            .inner
            .inputs
            .iter()
            .map(|input| CellInput(input))
            .collect()
    }

    fn outputs(&self) -> Vec<CellOutput> {
        self.0
            .inner
            .outputs
            .iter()
            .zip(self.0.inner.outputs_data.iter())
            .map(|(output, data)| {
                CellOutput(
                    output.clone(),
                    ckb_types::core::Capacity::bytes(data.len() as usize)
                        .expect("capacity overflow"),
                )
            })
            .collect()
    }

    fn witnesses(&self) -> Vec<String> {
        self.0
            .inner
            .witnesses
            .iter()
            .map(|witness| to_string(&witness).expect("serde"))
            .collect()
    }

    fn outputs_data(&self) -> Vec<Bytes> {
        self.0
            .inner
            .outputs_data
            .iter()
            .map(|output_data| Bytes(output_data.clone()))
            .collect()
    }

    fn hash(&self) -> String {
        to_string(&self.0.hash).expect("serde")
    }
}

pub struct CellDep<'a>(pub &'a ckb_jsonrpc_types::CellDep);

#[juniper::object(
    Context = Context,
)]
impl<'a> CellDep<'a> {
    fn out_point(&self) -> OutPoint {
        OutPoint(&self.0.out_point)
    }

    fn dep_type(&self) -> String {
        to_string(&self.0.dep_type).expect("serde")
    }
}

pub struct CellInput<'a>(pub &'a ckb_jsonrpc_types::CellInput);

#[juniper::object(
    Context = Context,
)]
impl<'a> CellInput<'a> {
    fn previous_output(&self) -> OutPoint {
        OutPoint(&self.0.previous_output)
    }

    fn since(&self) -> String {
        to_string(&self.0.since).expect("serde")
    }
}

pub struct OutPoint<'a>(pub &'a ckb_jsonrpc_types::OutPoint);

#[juniper::object(
    Context = Context,
)]
impl<'a> OutPoint<'a> {
    fn tx_hash(&self) -> String {
        to_string(&self.0.tx_hash).expect("serde")
    }

    fn index(&self) -> String {
        to_string(&self.0.index).expect("serde")
    }

    fn cell(&self, context: &Context) -> Option<CellOutput> {
        let cell_meta = context.get_cell_meta(&self.0.tx_hash.pack(), self.0.index.into());
        cell_meta.map(|meta| {
            CellOutput(
                meta.cell_output.into(),
                ckb_types::core::Capacity::bytes(meta.data_bytes as usize)
                    .expect("capacity overflow"),
            )
        })
    }

    fn cell_data(&self, context: &Context) -> Option<Bytes> {
        let cell_data = context.get_cell_data(&self.0.tx_hash.pack(), self.0.index.into());
        cell_data.map(|(data, _)| Bytes(ckb_jsonrpc_types::JsonBytes::from_bytes(data)))
    }
}

pub struct CellOutput(
    pub ckb_jsonrpc_types::CellOutput,
    pub ckb_types::core::Capacity,
);

#[juniper::object]
impl CellOutput {
    fn capacity(&self) -> String {
        to_string(&self.0.capacity).expect("serde")
    }

    fn occupied_capacity(&self) -> String {
        let output: packed::CellOutput = self.0.clone().into();
        let occupied_capacity = output.occupied_capacity(self.1).expect("occupied capacity");
        let capacity: ckb_jsonrpc_types::Capacity = occupied_capacity.into();
        to_string(&capacity).expect("serde")
    }

    fn lock(&self) -> Script {
        Script(&self.0.lock)
    }

    #[graphql(name = "type")]
    fn type_(&self) -> Option<Script> {
        match self.0.type_ {
            Some(ref s) => Some(Script(s)),
            None => None,
        }
    }
}

pub struct Script<'a>(pub &'a ckb_jsonrpc_types::Script);

#[juniper::object]
impl<'a> Script<'a> {
    fn args(&self) -> String {
        to_string(&self.0.args).expect("serde")
    }

    fn code_hash(&self) -> String {
        to_string(&self.0.code_hash).expect("serde")
    }

    fn hash_type(&self) -> String {
        to_string(&self.0.hash_type).expect("serde")
    }

    fn hash(&self) -> String {
        let packed_script: packed::Script = self.0.clone().into();
        let hash = packed_script.calc_script_hash();
        let hash: H256 = hash.unpack();
        to_string(&hash).expect("serde")
    }
}

pub struct Block(pub ckb_jsonrpc_types::BlockView);

#[juniper::object(
    Context = Context,
)]
impl Block {
    fn header(&self) -> Header {
        Header(self.0.header.clone())
    }

    fn uncles(&self) -> Vec<UncleBlock> {
        self.0
            .uncles
            .iter()
            .map(|uncle| UncleBlock(uncle))
            .collect()
    }

    fn transactions(&self) -> Vec<Transaction> {
        self.0
            .transactions
            .iter()
            .map(|tx| Transaction(tx.clone()))
            .collect()
    }

    fn proposals(&self) -> Vec<String> {
        self.0
            .proposals
            .iter()
            .map(|proposal| to_string(&proposal).expect("serde"))
            .collect()
    }

    fn hash(&self) -> String {
        to_string(&self.0.header.hash).expect("serde")
    }
}

pub struct UncleBlock<'a>(pub &'a ckb_jsonrpc_types::UncleBlockView);

#[juniper::object]
impl<'a> UncleBlock<'a> {
    fn header(&self) -> Header {
        Header(self.0.header.clone())
    }

    fn proposals(&self) -> Vec<String> {
        self.0
            .proposals
            .iter()
            .map(|proposal| to_string(&proposal).expect("serde"))
            .collect()
    }
}
