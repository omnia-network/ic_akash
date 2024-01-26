use crate::request::{Method, RequestMessage};
use serde::{Deserialize, Serialize};

/// Get information about a specific block
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Height of the block to request.
    ///
    /// If no height is provided, it will fetch results for the latest block.
    pub height: Option<Height>,
}

impl Request {
    /// Create a new request for information about a particular block
    pub fn new(height: Height) -> Self {
        Self {
            height: Some(height),
        }
    }
}

impl RequestMessage for Request {
    fn method(&self) -> Method {
        Method::Block
    }
}

/// Block height for a particular chain (i.e. number of blocks created since
/// the chain began)
///
/// A height of 0 represents a chain which has not yet produced a block.
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Height(u64);

impl Height {
    /// Create a new height
    pub fn new(height: u64) -> Self {
        Self(height)
    }

    /// Get inner integer value. Alternative to `.0` or `.into()`
    pub fn value(&self) -> u64 {
        self.0
    }
}
