use super::block::Height;
use crate::request::{Method, Request as RequestTrait, RequestMessage};
use crate::response::Response as ResponseTrait;
use bytes::Bytes;
use core::fmt::{self, Debug};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use subtle_encoding::{Encoding, Hex};
use tendermint_proto::v0_37::abci as pb;
use tendermint_proto::Protobuf;

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
    pub response: Info,
}

impl ResponseTrait for Response {}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(default, try_from = "pb::ResponseInfo", into = "pb::ResponseInfo")]
pub struct Info {
    /// Some arbitrary information.
    pub data: String,
    /// The application software semantic version.
    pub version: String,
    /// The application protocol version.
    pub app_version: u64,
    /// The latest block for which the app has called [`Commit`](super::super::Request::Commit).
    pub last_block_height: Height,
    /// The latest result of [`Commit`](super::super::Request::Commit).
    pub last_block_app_hash: AppHash,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

impl From<Info> for pb::ResponseInfo {
    fn from(info: Info) -> Self {
        Self {
            data: info.data,
            version: info.version,
            app_version: info.app_version,
            last_block_height: info.last_block_height.into(),
            last_block_app_hash: info.last_block_app_hash.into(),
        }
    }
}

impl TryFrom<pb::ResponseInfo> for Info {
    type Error = String;

    fn try_from(info: pb::ResponseInfo) -> Result<Self, String> {
        Ok(Self {
            data: info.data,
            version: info.version,
            app_version: info.app_version,
            last_block_height: info.last_block_height.try_into()?,
            last_block_app_hash: info.last_block_app_hash.try_into()?,
        })
    }
}

impl Protobuf<pb::ResponseInfo> for Info {}

/// AppHash is usually a SHA256 hash, but in reality it can be any kind of data
#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AppHash(Vec<u8>);

impl Protobuf<Vec<u8>> for AppHash {}

impl TryFrom<Vec<u8>> for AppHash {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, String> {
        Ok(AppHash(value))
    }
}
impl From<AppHash> for Vec<u8> {
    fn from(value: AppHash) -> Self {
        value.0
    }
}

impl TryFrom<Bytes> for AppHash {
    type Error = String;

    fn try_from(value: Bytes) -> Result<Self, String> {
        Ok(AppHash(value.to_vec()))
    }
}
impl From<AppHash> for Bytes {
    fn from(value: AppHash) -> Self {
        value.0.into()
    }
}

impl AppHash {
    /// Return the hash bytes as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Decode a `Hash` from upper-case hexadecimal
    pub fn from_hex_upper(s: &str) -> Result<Self, String> {
        if s.len() % 2 != 0 {
            return Err(String::from("invalid app hash length"));
        }
        let mut h = vec![0; s.len() / 2];
        Hex::upper_case()
            .decode_to_slice(s.as_bytes(), &mut h)
            .map_err(|e| e.to_string())?;
        Ok(AppHash(h))
    }
}

impl AsRef<[u8]> for AppHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Debug for AppHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AppHash({})",
            Hex::upper_case().encode_to_string(&self.0).unwrap()
        )
    }
}

impl Display for AppHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Hex::upper_case().encode_to_string(&self.0).unwrap()
        )
    }
}

impl FromStr for AppHash {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_hex_upper(s)
    }
}
