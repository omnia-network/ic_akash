use crate::{
    request::{Method, Request as RequestTrait, RequestMessage},
    serializers,
};
use bytes::Bytes;
use core::{fmt, num::NonZeroU32};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use subtle_encoding::{Encoding, Hex};
use tendermint::block::Height;

use serde::{
    de::{Deserializer, Visitor},
    Serializer,
};
use tendermint_proto::Protobuf;

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

impl RequestTrait for Request {
    type Response = Response;
}

// impl SimpleRequest for Request {
//     type Output = Response;
// }

/// ABCI query response wrapper
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Response {
    /// ABCI query results
    pub response: AbciQuery,
}

impl crate::Response for Response {}

/// ABCI query results
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct AbciQuery {
    /// Response code
    pub code: Code,

    /// Log value
    pub log: String,

    /// Info value
    #[serde(default = "String::new")]
    pub info: String,

    /// Index
    #[serde(with = "serializers::from_str")]
    pub index: i64,

    /// Key
    #[serde(default, with = "serializers::base64string")]
    pub key: Vec<u8>,

    /// Value
    #[serde(default, with = "serializers::base64string")]
    pub value: Vec<u8>,

    /// Proof (might be explicit null)
    #[serde(alias = "proofOps")]
    pub proof: Option<ProofOps>,

    /// Block height
    pub height: Height,

    /// Codespace
    #[serde(default = "String::new")]
    pub codespace: String,
}

/// ABCI application response codes.
///
/// These presently use 0 for success and non-zero for errors:
///
/// <https://tendermint.com/docs/spec/abci/abci.html#errors>
///
/// Note that in the future there may potentially be non-zero success codes.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Default)]
pub enum Code {
    /// Success
    #[default]
    Ok,

    /// Error codes
    Err(NonZeroU32),
}

impl Code {
    /// Was the response OK?
    pub fn is_ok(self) -> bool {
        match self {
            Code::Ok => true,
            Code::Err(_) => false,
        }
    }

    /// Was the response an error?
    pub fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Get the integer error value for this code
    pub fn value(self) -> u32 {
        u32::from(self)
    }
}

impl From<u32> for Code {
    fn from(value: u32) -> Code {
        match NonZeroU32::new(value) {
            Some(value) => Code::Err(value),
            None => Code::Ok,
        }
    }
}

impl From<Code> for u32 {
    fn from(code: Code) -> u32 {
        match code {
            Code::Ok => 0,
            Code::Err(err) => err.get(),
        }
    }
}

impl Serialize for Code {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CodeVisitor;

        impl<'de> Visitor<'de> for CodeVisitor {
            type Value = Code;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Code::from(val as u32))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(CodeVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "RawProof", into = "RawProof")]
pub struct Proof {
    // Total number of items.
    pub total: u64,
    // Index of the item to prove.
    pub index: u64,
    // Hash of item value.
    pub leaf_hash: Hash,
    // Hashes from leaf's sibling to a root's child.
    pub aunts: Vec<Hash>,
}

/// Merkle proof defined by the list of ProofOps
/// <https://github.com/tendermint/tendermint/blob/c8483531d8e756f7fbb812db1dd16d841cdf298a/crypto/merkle/merkle.proto#L26>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProofOps {
    /// The list of ProofOps
    pub ops: Vec<ProofOp>,
}

/// ProofOp defines an operation used for calculating Merkle root
/// The data could be arbitrary format, providing necessary data
/// for example neighbouring node hash
/// <https://github.com/tendermint/tendermint/blob/c8483531d8e756f7fbb812db1dd16d841cdf298a/crypto/merkle/merkle.proto#L19>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProofOp {
    /// Type of the ProofOp
    #[serde(alias = "type")]
    pub field_type: String,
    /// Key of the ProofOp
    #[serde(default, with = "serializers::base64string")]
    pub key: Vec<u8>,
    /// Actual data
    #[serde(default, with = "serializers::base64string")]
    pub data: Vec<u8>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

use tendermint_proto::v0_37::crypto::{
    Proof as RawProof, ProofOp as RawProofOp, ProofOps as RawProofOps,
};

impl Protobuf<RawProof> for Proof {}

impl TryFrom<RawProof> for Proof {
    type Error = String;

    fn try_from(message: RawProof) -> Result<Self, Self::Error> {
        Ok(Self {
            total: message
                .total
                .try_into()
                .map_err(|_| "negative proof total")?,
            index: message
                .index
                .try_into()
                .map_err(|_| "negative proof index")?,
            leaf_hash: message.leaf_hash.try_into()?,
            aunts: message
                .aunts
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Proof> for RawProof {
    fn from(value: Proof) -> Self {
        Self {
            total: value
                .total
                .try_into()
                .expect("number of items is too large"),
            index: value.index.try_into().expect("index is too large"),
            leaf_hash: value.leaf_hash.into(),
            aunts: value.aunts.into_iter().map(Into::into).collect(),
        }
    }
}

impl Protobuf<RawProofOp> for ProofOp {}

impl TryFrom<RawProofOp> for ProofOp {
    type Error = String;

    fn try_from(value: RawProofOp) -> Result<Self, Self::Error> {
        Ok(Self {
            field_type: value.r#type,
            key: value.key,
            data: value.data,
        })
    }
}

impl From<ProofOp> for RawProofOp {
    fn from(value: ProofOp) -> Self {
        RawProofOp {
            r#type: value.field_type,
            key: value.key,
            data: value.data,
        }
    }
}

impl Protobuf<RawProofOps> for ProofOps {}

impl TryFrom<RawProofOps> for ProofOps {
    type Error = String;

    fn try_from(value: RawProofOps) -> Result<Self, Self::Error> {
        let ops: Result<Vec<ProofOp>, _> = value.ops.into_iter().map(ProofOp::try_from).collect();

        Ok(Self { ops: ops? })
    }
}

impl From<ProofOps> for RawProofOps {
    fn from(value: ProofOps) -> Self {
        let ops: Vec<RawProofOp> = value.ops.into_iter().map(RawProofOp::from).collect();

        RawProofOps { ops }
    }
}

/// Output size for the SHA-256 hash function
pub const SHA256_HASH_SIZE: usize = 32;

/// Hash algorithms
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Algorithm {
    /// SHA-256
    Sha256,
}

/// Hash digests
#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum Hash {
    /// SHA-256 hashes
    Sha256([u8; SHA256_HASH_SIZE]),
    /// Empty hash
    #[default]
    None,
}

impl Protobuf<Vec<u8>> for Hash {}

/// Default conversion from `Vec<u8>` is SHA256 Hash or `None`
impl TryFrom<Vec<u8>> for Hash {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Hash::None);
        }
        Hash::from_bytes(Algorithm::Sha256, &value)
    }
}

impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        match value {
            Hash::Sha256(s) => s.to_vec(),
            Hash::None => vec![],
        }
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        match self {
            Hash::Sha256(ref h) => h.as_ref(),
            Hash::None => &[],
        }
    }
}

impl From<Hash> for Bytes {
    fn from(h: Hash) -> Self {
        Self::copy_from_slice(h.as_ref())
    }
}

impl TryFrom<Bytes> for Hash {
    type Error = String;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Self::from_bytes(Algorithm::Sha256, value.as_ref())
    }
}

impl Hash {
    /// Create a new `Hash` with the given algorithm type
    pub fn from_bytes(alg: Algorithm, bytes: &[u8]) -> Result<Hash, String> {
        if bytes.is_empty() {
            return Ok(Hash::None);
        }
        match alg {
            Algorithm::Sha256 => {
                if bytes.len() == SHA256_HASH_SIZE {
                    let mut h = [0u8; SHA256_HASH_SIZE];
                    h.copy_from_slice(bytes);
                    Ok(Hash::Sha256(h))
                } else {
                    Err(String::from("invalid hash size"))
                }
            }
        }
    }

    /// Decode a `Hash` from upper-case hexadecimal
    pub fn from_hex_upper(alg: Algorithm, s: &str) -> Result<Hash, String> {
        if s.is_empty() {
            return Ok(Hash::None);
        }
        match alg {
            Algorithm::Sha256 => {
                let mut h = [0u8; SHA256_HASH_SIZE];
                Hex::upper_case()
                    .decode_to_slice(s.as_bytes(), &mut h)
                    .map_err(|_| "subtle encoding")?;
                Ok(Hash::Sha256(h))
            }
        }
    }

    /// Return the digest algorithm used to produce this hash
    pub fn algorithm(self) -> Algorithm {
        match self {
            Hash::Sha256(_) => Algorithm::Sha256,
            Hash::None => Algorithm::Sha256,
        }
    }

    /// Borrow the `Hash` as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Hash::Sha256(ref h) => h.as_ref(),
            Hash::None => &[],
        }
    }

    /// Convenience function to check for Hash::None
    pub fn is_empty(&self) -> bool {
        self == &Hash::None
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hash::Sha256(_) => write!(f, "Hash::Sha256({self})"),
            Hash::None => write!(f, "Hash::None"),
        }
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = match self {
            Hash::Sha256(ref h) => Hex::upper_case().encode_to_string(h).unwrap(),
            Hash::None => String::new(),
        };

        write!(f, "{hex}")
    }
}

impl FromStr for Hash {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Self::from_hex_upper(Algorithm::Sha256, s)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hex = <&str>::deserialize(deserializer)?;
        Ok(Self::from_str(hex).unwrap())
    }
}

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

/// Serialization/deserialization for `Hash` that allows for empty hashes.
pub mod allow_empty {
    use super::*;

    /// Serialize [`Hash`](enum@crate::hash::Hash) into a string.
    pub fn serialize<S>(value: &Hash, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }

    /// Deserialize [`Hash`](enum@crate::hash::Hash) from a string, allowing for
    /// empty hashes.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = <&str>::deserialize(deserializer)?;
        Hash::from_str(hex).map_err(serde::de::Error::custom)
    }
}
