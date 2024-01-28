//! JSON-RPC response types
use crate::{uuid::Uuid, version::Version};
use core::fmt::Display;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
use std::io::Read;

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
    id: Uuid,

    /// Results of request (if successful)
    result: Option<R>,

    /// Error message if unsuccessful
    error: Option<ResponseError>,
}

impl<R> Wrapper<R> {
    /// Get JSON-RPC version
    pub fn version(&self) -> &Version {
        &self.jsonrpc
    }

    /// Get JSON-RPC ID
    #[allow(dead_code)]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Convert this wrapper into the underlying error, if any
    pub fn into_error(self) -> Option<String> {
        self.error.map(|e| e.to_string())
    }

    /// Convert this wrapper into a result type
    pub fn into_result(self) -> Result<R, String> {
        // Ensure we're using a supported RPC version
        self.version().ensure_supported()?;

        if let Some(e) = self.error {
            Err(format!("JSON-RPC error: {e:?}"))
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(String::from("malformed JSON-RPC response"))
        }
    }

    pub fn new_with_id(id: Uuid, result: Option<R>, error: Option<ResponseError>) -> Self {
        Self {
            jsonrpc: Version::current(),
            id,
            result,
            error,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseError {
    /// Error code
    code: Code,

    /// Error message
    message: String,

    /// Additional data about the error
    data: Option<String>,
}

// /// Tendermint RPC error codes.
// ///
/// See `func RPC*Error()` definitions in:
/// <https://github.com/tendermint/tendermint/blob/main/rpc/jsonrpc/types/types.go>
#[derive(Copy, Clone, Debug, Eq, thiserror::Error, Hash, PartialEq, PartialOrd, Ord)]
pub enum Code {
    /// Low-level HTTP error
    #[error("HTTP error")]
    HttpError,

    /// Low-level WebSocket error
    #[error("WebSocket Error")]
    WebSocketError,

    /// An internal error occurred within the client.
    ///
    /// This is an error unique to this client, and is not available in the
    /// [Go client].
    ///
    /// [Go client]: https://github.com/tendermint/tendermint/tree/main/rpc/jsonrpc/client
    #[error("Client internal error")]
    ClientInternalError,

    /// Parse error i.e. invalid JSON (-32700)
    #[error("Parse error. Invalid JSON")]
    ParseError,

    /// Invalid request (-32600)
    #[error("Invalid Request")]
    InvalidRequest,

    /// Method not found error (-32601)
    #[error("Method not found")]
    MethodNotFound,

    /// Invalid parameters (-32602)
    #[error("Invalid params")]
    InvalidParams,

    /// Internal RPC server error (-32603)
    #[error("Internal error")]
    InternalError,

    /// Server error (-32000)
    #[error("Server error")]
    ServerError,

    /// Other error types
    #[error("Error (code: {})", 0)]
    Other(i32),
}

impl Code {
    /// Get the integer error value for this code
    pub fn value(self) -> i32 {
        i32::from(self)
    }
}

impl From<i32> for Code {
    fn from(value: i32) -> Code {
        match value {
            0 => Code::HttpError,
            1 => Code::WebSocketError,
            2 => Code::ClientInternalError,
            -32700 => Code::ParseError,
            -32600 => Code::InvalidRequest,
            -32601 => Code::MethodNotFound,
            -32602 => Code::InvalidParams,
            -32603 => Code::InternalError,
            -32000 => Code::ServerError,
            other => Code::Other(other),
        }
    }
}

impl From<Code> for i32 {
    fn from(code: Code) -> i32 {
        match code {
            Code::HttpError => 0,
            Code::WebSocketError => 1,
            Code::ClientInternalError => 2,
            Code::ParseError => -32700,
            Code::InvalidRequest => -32600,
            Code::MethodNotFound => -32601,
            Code::InvalidParams => -32602,
            Code::InternalError => -32603,
            Code::ServerError => -32000,
            Code::Other(other) => other,
        }
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.data {
            Some(data) => write!(
                f,
                "{}: {} (code: {})",
                self.message,
                data,
                self.code.value()
            ),
            None => write!(f, "{} (code: {})", self.message, self.code.value()),
        }
    }
}

impl ResponseError {
    /// Create a new RPC error
    pub fn new(code: Code, data: Option<String>) -> ResponseError {
        let message = code.to_string();

        ResponseError {
            code,
            message,
            data,
        }
    }

    // / Create a low-level HTTP error
    pub fn http_error(message: impl Into<String>) -> ResponseError {
        ResponseError {
            code: Code::HttpError,
            message: message.into(),
            data: None,
        }
    }

    /// Create a new invalid parameter error
    pub fn invalid_params(data: &str) -> ResponseError {
        ResponseError::new(Code::InvalidParams, Some(data.to_string()))
    }

    /// Create a new websocket error
    pub fn websocket_error(cause: impl Into<String>) -> ResponseError {
        ResponseError::new(Code::WebSocketError, Some(cause.into()))
    }

    /// Create a new method-not-found error
    pub fn method_not_found(name: &str) -> ResponseError {
        ResponseError::new(Code::MethodNotFound, Some(name.to_string()))
    }

    /// Create a new parse error
    pub fn parse_error<E>(error: E) -> ResponseError
    where
        E: Display,
    {
        ResponseError::new(Code::ParseError, Some(error.to_string()))
    }

    /// Create a new server error
    pub fn server_error<D>(data: D) -> ResponseError
    where
        D: Display,
    {
        ResponseError::new(Code::ServerError, Some(data.to_string()))
    }

    /// An internal error occurred within the client.
    pub fn client_internal_error(cause: impl Into<String>) -> ResponseError {
        ResponseError::new(Code::ClientInternalError, Some(cause.into()))
    }

    /// Obtain the `rpc::error::Code` for this error
    pub fn code(&self) -> Code {
        self.code
    }

    /// Borrow the error message (if available)
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Optional additional error message (if available)
    pub fn data(&self) -> Option<&str> {
        self.data.as_ref().map(AsRef::as_ref)
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Code::from(i32::deserialize(deserializer)?))
    }
}

impl Serialize for Code {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}
