use core::{
    fmt::{self, Display},
    str::FromStr,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde::{de::Error as _, Deserializer, Serializer};

use crate::uuid::Uuid;

/// Supported JSON-RPC version
const SUPPORTED_VERSION: &str = "2.0";

/// JSON-RPC version
// TODO(tarcieri): add restrictions/validations on these formats? Use an `enum`?
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Version(String);

impl Version {
    /// Get the currently supported JSON-RPC version
    pub fn current() -> Self {
        Version(SUPPORTED_VERSION.to_owned())
    }

    /// Is this JSON-RPC version supported?
    pub fn is_supported(&self) -> bool {
        self.0.eq(SUPPORTED_VERSION)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Version(s.to_owned()))
    }
}

/// JSON-RPC request methods.
///
/// Serialized as the "method" field of JSON-RPC/HTTP requests.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Method {
    /// Get ABCI info
    AbciInfo,

    /// Get ABCI query
    AbciQuery,

    /// Get block info
    Block,

    /// Get block info by hash
    BlockByHash,

    /// Get ABCI results for a particular block
    BlockResults,

    /// Search for blocks by their BeginBlock and EndBlock events
    BlockSearch,

    /// Get blockchain info
    Blockchain,

    /// Broadcast transaction asynchronously
    BroadcastTxAsync,

    /// Broadcast transaction synchronously
    BroadcastTxSync,

    /// Broadcast transaction commit
    BroadcastTxCommit,

    /// Get commit info for a block
    Commit,

    /// Get consensus parameters
    ConsensusParams,

    /// Get consensus state
    ConsensusState,

    /// Get genesis file
    Genesis,

    /// Get block header
    Header,

    /// Get block header by hash
    HeaderByHash,

    /// Get health info
    Health,

    /// Get network info
    NetInfo,

    /// Get node status
    Status,

    /// Find transaction by hash
    Tx,

    /// Search for transactions with their results
    TxSearch,

    /// Get validator info for a block
    Validators,

    /// Subscribe to events
    Subscribe,

    /// Unsubscribe from events
    Unsubscribe,

    /// Broadcast evidence
    BroadcastEvidence,
}

impl Method {
    /// Get a static string which represents this method name
    pub fn as_str(self) -> &'static str {
        match self {
            Method::AbciInfo => "abci_info",
            Method::AbciQuery => "abci_query",
            Method::Block => "block",
            Method::BlockByHash => "block_by_hash",
            Method::BlockResults => "block_results",
            Method::BlockSearch => "block_search",
            Method::Blockchain => "blockchain",
            Method::BroadcastEvidence => "broadcast_evidence",
            Method::BroadcastTxAsync => "broadcast_tx_async",
            Method::BroadcastTxSync => "broadcast_tx_sync",
            Method::BroadcastTxCommit => "broadcast_tx_commit",
            Method::Commit => "commit",
            Method::ConsensusParams => "consensus_params",
            Method::ConsensusState => "consensus_state",
            Method::Genesis => "genesis",
            Method::Header => "header",
            Method::HeaderByHash => "header_by_hash",
            Method::Health => "health",
            Method::NetInfo => "net_info",
            Method::Status => "status",
            Method::Subscribe => "subscribe",
            Method::Tx => "tx",
            Method::TxSearch => "tx_search",
            Method::Unsubscribe => "unsubscribe",
            Method::Validators => "validators",
        }
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Ok(match s {
            "abci_info" => Method::AbciInfo,
            "abci_query" => Method::AbciQuery,
            "block" => Method::Block,
            "block_by_hash" => Method::BlockByHash,
            "block_results" => Method::BlockResults,
            "header" => Method::Header,
            "header_by_hash" => Method::HeaderByHash,
            "block_search" => Method::BlockSearch,
            "blockchain" => Method::Blockchain,
            "broadcast_evidence" => Method::BroadcastEvidence,
            "broadcast_tx_async" => Method::BroadcastTxAsync,
            "broadcast_tx_sync" => Method::BroadcastTxSync,
            "broadcast_tx_commit" => Method::BroadcastTxCommit,
            "commit" => Method::Commit,
            "consensus_params" => Method::ConsensusParams,
            "consensus_state" => Method::ConsensusState,
            "genesis" => Method::Genesis,
            "health" => Method::Health,
            "net_info" => Method::NetInfo,
            "status" => Method::Status,
            "subscribe" => Method::Subscribe,
            "tx" => Method::Tx,
            "tx_search" => Method::TxSearch,
            "unsubscribe" => Method::Unsubscribe,
            "validators" => Method::Validators,
            other => return Err(other.to_string()),
        })
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Method {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(format!("{e}")))
    }
}

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

/// JSON-RPC request wrapper (i.e. message envelope)
#[derive(Debug, Deserialize, Serialize)]
pub struct Wrapper<R> {
    /// JSON-RPC version
    jsonrpc: Version,

    /// Identifier included in request
    id: Uuid,

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
        Self::new_with_id(Uuid::new().await.unwrap(), request)
    }

    pub(crate) fn new_with_id(id: Uuid, request: R) -> Self {
        Self {
            jsonrpc: Version::current(),
            id,
            method: request.method(),
            params: request,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn params(&self) -> &R {
        &self.params
    }

    pub fn into_json(self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
