//! JSON-RPC response types

use std::io::Read;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{id::Id, response_error::ResponseError, version::Version};

/// JSON-RPC responses
pub trait Response: DeserializeOwned + Sized {
    /// Parse a JSON-RPC response from a JSON string
    fn from_string(response: impl AsRef<[u8]>) -> Result<Self, String> {
        let wrapper: Wrapper<Self> =
            serde_json::from_slice(response.as_ref()).map_err(|e| e.to_string())?;
        wrapper.into_result()
    }

    /// Parse a JSON-RPC response from an `io::Reader`
    fn from_reader(reader: impl Read) -> Result<Self, String> {
        let wrapper: Wrapper<Self> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
        wrapper.into_result()
    }
}

/// JSON-RPC response wrapper (i.e. message envelope)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Wrapper<R> {
    /// JSON-RPC version
    jsonrpc: Version,

    /// Identifier included in request
    id: Id,

    /// Results of request (if successful)
    result: Option<R>,

    /// Error message if unsuccessful
    error: Option<ResponseError>,
}

#[allow(dead_code)]
impl<R> Wrapper<R> {
    /// Get JSON-RPC version
    pub fn version(&self) -> &Version {
        &self.jsonrpc
    }

    /// Get JSON-RPC ID
    #[allow(dead_code)]
    pub fn id(&self) -> &Id {
        &self.id
    }

    /// Convert this wrapper into the underlying error, if any
    pub fn into_error(self) -> Option<String> {
        self.error.map(|e| format!("response error: {}", e))
    }

    /// Convert this wrapper into a result type
    pub fn into_result(self) -> Result<R, String> {
        // Ensure we're using a supported RPC version
        self.version().ensure_supported()?;

        if let Some(e) = self.error {
            Err(format!("response error: {}", e))
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err("server returned malformatted JSON (no 'result' or 'error')".to_string())
        }
    }

    pub fn new_with_id(id: Id, result: Option<R>, error: Option<ResponseError>) -> Self {
        Self {
            jsonrpc: Version::current(),
            id,
            result,
            error,
        }
    }
}
