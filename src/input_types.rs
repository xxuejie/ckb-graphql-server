use ckb_jsonrpc_types::OutPoint as RpcOutPoint;
use serde_json::from_str as from_json_str;
use serde_plain::from_str;
use std::convert::TryFrom;

#[derive(juniper::GraphQLInputObject)]
pub struct InputOutPoint {
    tx_hash: String,
    index: String,
}

impl TryFrom<&InputOutPoint> for RpcOutPoint {
    type Error = String;

    fn try_from(o: &InputOutPoint) -> Result<RpcOutPoint, String> {
        Ok(RpcOutPoint {
            tx_hash: from_str(&o.tx_hash).map_err(|e| e.to_string())?,
            index: from_json_str(&format!("\"{}\"", o.index)).map_err(|e| e.to_string())?,
        })
    }
}
