use core::{
    fmt::{self, Display},
    str::FromStr,
};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use serde::{Deserialize, Serialize};

use serde::{de::Error as _, Deserializer, Serializer};

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

/// Serialize into base64string, deserialize from base64string
pub mod base64string {
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::base64;

    /// Deserialize base64string into `Vec<u8>`
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        Vec<u8>: Into<T>,
    {
        let s = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        let v = base64::decode(s).map_err(serde::de::Error::custom)?;
        Ok(v.into())
    }

    /// Deserialize base64string into String
    pub fn deserialize_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        String::from_utf8(base64::decode(s).map_err(serde::de::Error::custom)?)
            .map_err(serde::de::Error::custom)
    }

    /// Serialize from T into base64string
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let base64_bytes = base64::encode(value.as_ref());
        let base64_string = String::from_utf8(base64_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&base64_string)
    }
}

/// `/broadcast_tx_sync`: returns with the response from `CheckTx`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    /// Transaction to broadcast
    // #[serde(with = "base64string")]
    // pub tx: Vec<u8>,
    pub height: Option<u64>,
}

impl Request {
    /// Create a new sync transaction broadcast RPC request
    pub fn new(tx: impl Into<Vec<u8>>) -> Request {
        // Request { tx: tx.into() }
        Request { height: None }
    }
}

/// JSON-RPC request wrapper (i.e. message envelope)
#[derive(Debug, Deserialize, Serialize)]
pub struct Wrapper {
    /// JSON-RPC version
    jsonrpc: Version,

    /// Identifier included in request
    id: u64,

    /// Request method
    method: Method,

    /// Request parameters (i.e. request object)
    params: Request,
}

impl Wrapper {
    /// Create a new request wrapper from the given request.
    ///
    /// The ID of the request is set to a random [UUIDv4] value.
    ///
    /// [UUIDv4]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)
    pub fn new(request: Request) -> Self {
        Self::new_with_id(0, request)
    }

    pub(crate) fn new_with_id(id: u64, request: Request) -> Self {
        Self {
            jsonrpc: Version::current(),
            id,
            method: Method::Block,
            params: request,
        }
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }

    pub fn params(&self) -> &Request {
        &self.params
    }

    pub fn into_json(self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[ic_cdk::update]
async fn broadcast_tx_sync(tx_raw: String) -> Result<(), String> {
    let tx = hex::decode(tx_raw).map_err(|e| e.to_string())?;

    let request = Request::new(tx);

    let request_body = Wrapper::new(request).into_json().into_bytes();

    ic_cdk::api::print(format!("{:?}", request_body));

    // let host = "putsreq.com";
    let url = "https://rpc.sandbox-01.aksh.pw";

    // 2.2 prepare headers for the system http_request call
    //Note that `HttpHeader` is declared in line 4
    let request_headers = vec![
        // HttpHeader {
        //     name: "Host".to_string(),
        //     value: format!("{host}:443"),
        // },
        // HttpHeader {
        //     name: "User-Agent".to_string(),
        //     value: "demo_HTTP_POST_canister".to_string(),
        // },
        // //For the purposes of this exercise, Idempotency-Key" is hard coded, but in practice
        // //it should be generated via code and unique to each POST request. Common to create helper methods for this
        // HttpHeader {
        //     name: "Idempotency-Key".to_string(),
        //     value: "UUID-123456789".to_string(),
        // },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: None, //optional for request
        method: HttpMethod::POST,
        headers: request_headers,
        body: Some(request_body),
        transform: None,
    };

    ic_cdk::api::print(format!("{:?}", request));

    match http_request(request).await {
        //4. DECODE AND RETURN THE RESPONSE

        //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
        Ok((response,)) => {
            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //You need to decode that Vec<u8> that is the body into readable text.
            //To do this:
            //  1. Call `String::from_utf8()` on response.body
            //  3. Use a switch to explicitly call out both cases of decoding the Blob into ?Text
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");
            ic_cdk::api::print(format!("{:?}", str_body));

            //The API response will looks like this:
            // { successful: true }

            //Return the body as a string and end the method
            let result: String = format!(
                "{}. See more info of the request sent at: {}/inspect",
                str_body, url
            );
            result
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            message
        }
    };

    Ok(())
}
