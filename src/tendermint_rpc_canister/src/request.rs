//! JSON-RPC requests

use core::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{id::Id, method::Method, version::Version};

/// Serialization for JSON-RPC requests
pub trait RequestMessage: DeserializeOwned + Serialize + Sized {
    /// Request method
    fn method(&self) -> Method;

    /// Serialize this request as JSON
    async fn into_json(self) -> String {
        Wrapper::new(self).await.into_json()
    }

    /// Parse a JSON-RPC request from a JSON string.
    fn from_string(s: impl AsRef<[u8]>) -> Result<Self, String> {
        let wrapper: Wrapper<Self> =
            serde_json::from_slice(s.as_ref()).map_err(|e| e.to_string())?;
        Ok(wrapper.params)
    }
}

/// JSON-RPC requests
pub trait Request: RequestMessage + Send {
    /// Response type for this command
    type Response: super::response::Response;
}

/// JSON-RPC request wrapper (i.e. message envelope)
#[derive(Debug, Deserialize, Serialize)]
pub struct Wrapper<R> {
    /// JSON-RPC version
    jsonrpc: Version,

    /// Identifier included in request
    id: Id,

    /// Request method
    method: Method,

    /// Request parameters (i.e. request object)
    params: R,
}

impl<R> Wrapper<R>
where
    R: RequestMessage,
{
    /// Create a new request wrapper from the given request.
    ///
    /// The ID of the request is set to a random [UUIDv4] value.
    ///
    /// [UUIDv4]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)
    pub async fn new(request: R) -> Self {
        Self::new_with_id(Id::uuid_v4().await, request)
    }

    pub(crate) fn new_with_id(id: Id, request: R) -> Self {
        Self {
            jsonrpc: Version::current(),
            id,
            method: request.method(),
            params: request,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn params(&self) -> &R {
        &self.params
    }

    pub fn into_json(self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
