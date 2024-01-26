use serde::{Deserialize, Serialize};

use crate::request::{Method, RequestMessage};

/// Request ABCI information from a node
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request;

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::AbciInfo
    }
}
