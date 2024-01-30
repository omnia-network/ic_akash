//! `/abci_info` endpoint JSON-RPC wrapper

use serde::{Deserialize, Serialize};

use crate::{
    method::Method,
    request::{Request as RequestTrait, RequestMessage},
};

/// Request ABCI information from a node
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::AbciInfo
    }
}

impl RequestTrait for Request {
    type Response = Response;
}

/// ABCI information response
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    /// ABCI info
    pub response: tendermint::abci::response::Info,
}

impl crate::Response for Response {}
