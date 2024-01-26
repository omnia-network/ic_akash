use crate::{
    request::{Method, RequestMessage},
    serializers::base64string,
};
use serde::{Deserialize, Serialize};

/// `/broadcast_tx_sync`: returns with the response from `CheckTx`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Transaction to broadcast
    #[serde(with = "base64string")]
    pub tx: Vec<u8>,
}

impl Request {
    /// Create a new sync transaction broadcast RPC request
    pub fn new(tx: impl Into<Vec<u8>>) -> Request {
        Request { tx: tx.into() }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::BroadcastTxSync
    }
}
