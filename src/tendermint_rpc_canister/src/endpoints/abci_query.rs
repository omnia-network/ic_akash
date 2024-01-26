use super::block::Height;
use crate::{
    request::{Method, RequestMessage},
    serializers,
};
use serde::{Deserialize, Serialize};

/// Query the ABCI application for information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Path to the data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Data to query
    #[serde(with = "serializers::hexstring")]
    pub data: Vec<u8>,

    /// Block height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Height>,

    /// Include proof in response
    #[serde(default)]
    pub prove: bool,
}

impl Request {
    /// Create a new ABCI query request
    pub fn new<D>(path: Option<String>, data: D, height: Option<Height>, prove: bool) -> Self
    where
        D: Into<Vec<u8>>,
    {
        Self {
            path,
            data: data.into(),
            height,
            prove,
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::AbciQuery
    }
}
