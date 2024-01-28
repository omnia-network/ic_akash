use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display},
    str::FromStr,
};

use crate::request::{Method, RequestMessage};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tendermint_proto::Protobuf;

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
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Height(u64);

impl Protobuf<i64> for Height {}

impl TryFrom<i64> for Height {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Height(value.try_into().map_err(|_| "negative height")?))
    }
}

impl From<Height> for i64 {
    fn from(value: Height) -> Self {
        value.value() as i64 // does not overflow. The value is <= i64::MAX
    }
}

impl TryFrom<u64> for Height {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // Make sure the u64 value can be converted safely to i64
        let _ival: i64 = value.try_into().map_err(|_| "integer overflow")?;

        Ok(Height(value))
    }
}

impl From<Height> for u64 {
    fn from(value: Height) -> Self {
        value.value()
    }
}

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Height(value as u64)
    }
}

impl From<u16> for Height {
    fn from(value: u16) -> Self {
        Height(value as u64)
    }
}

impl From<u8> for Height {
    fn from(value: u8) -> Self {
        Height(value as u64)
    }
}

impl Height {
    /// Create a new height
    pub fn new(height: u64) -> Self {
        Self(height)
    }

    /// Get inner integer value. Alternative to `.0` or `.into()`
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Increment the block height by 1
    pub fn increment(self) -> Self {
        Height::try_from(self.0.checked_add(1).expect("height overflow")).unwrap()
    }
}

impl Debug for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block::Height({})", self.0)
    }
}

impl Default for Height {
    fn default() -> Self {
        Height(1)
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Height::try_from(s.parse::<u64>().map_err(|e| e.to_string())?)
    }
}

impl<'de> Deserialize<'de> for Height {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_str(&String::deserialize(deserializer)?).unwrap())
    }
}

impl Serialize for Height {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        i64::from(*self).to_string().serialize(serializer)
    }
}

/// Parse `block::Height` from a type
pub trait ParseHeight {
    /// Parse `block::Height`, or return an `Error` if parsing failed
    fn parse_block_height(&self) -> Result<Height, String>;
}
