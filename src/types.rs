use ckb_jsonrpc_types;
use serde_plain::to_string;

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

#[juniper::object]
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
            .map(|output| CellOutput(output))
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

    fn outputs_data(&self) -> Vec<String> {
        self.0
            .inner
            .outputs_data
            .iter()
            .map(|output_data| to_string(&output_data).expect("serde"))
            .collect()
    }

    fn hash(&self) -> String {
        to_string(&self.0.hash).expect("serde")
    }
}

pub struct CellDep<'a>(pub &'a ckb_jsonrpc_types::CellDep);

#[juniper::object]
impl<'a> CellDep<'a> {
    fn out_point(&self) -> OutPoint {
        OutPoint(&self.0.out_point)
    }

    fn dep_type(&self) -> String {
        to_string(&self.0.dep_type).expect("serde")
    }
}

pub struct CellInput<'a>(pub &'a ckb_jsonrpc_types::CellInput);

#[juniper::object]
impl<'a> CellInput<'a> {
    fn previous_output(&self) -> OutPoint {
        OutPoint(&self.0.previous_output)
    }

    fn since(&self) -> String {
        to_string(&self.0.since).expect("serde")
    }
}

pub struct OutPoint<'a>(pub &'a ckb_jsonrpc_types::OutPoint);

#[juniper::object]
impl<'a> OutPoint<'a> {
    fn tx_hash(&self) -> String {
        to_string(&self.0.tx_hash).expect("serde")
    }

    fn index(&self) -> String {
        to_string(&self.0.index).expect("serde")
    }
}

pub struct CellOutput<'a>(pub &'a ckb_jsonrpc_types::CellOutput);

#[juniper::object]
impl<'a> CellOutput<'a> {
    fn capacity(&self) -> String {
        to_string(&self.0.capacity).expect("serde")
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
}

pub struct Block(pub ckb_jsonrpc_types::BlockView);

#[juniper::object]
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
