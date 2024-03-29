//! Serialize/deserialize bytes (`Vec<u8>`) type

#[allow(dead_code)]
/// Serialize into hexstring, deserialize from hexstring
pub mod hexstring {
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::hex;

    /// Deserialize a hex-encoded string into `Vec<u8>`
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        hex::decode_upper(&string)
            .or_else(|_| hex::decode(&string))
            .map_err(serde::de::Error::custom)
    }

    /// Serialize from a byte slice into a hex-encoded string.
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let hex_bytes = hex::encode_upper(value.as_ref());
        let hex_string = String::from_utf8(hex_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&hex_string)
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub mod from_str {
    use core::fmt::Display;
    use core::str::FromStr;
    use std::borrow::Cow;

    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    /// Deserialize string into T
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        <Cow<'_, str>>::deserialize(deserializer)?
            .parse::<T>()
            .map_err(D::Error::custom)
    }

    /// Serialize from T into string
    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        value.to_string().serialize(serializer)
    }
}

pub mod tx_hash_base64 {
    use serde::{Deserialize, Deserializer, Serializer};
    use subtle_encoding::base64;
    use tendermint::{hash::Algorithm, Hash};

    /// Deserialize a base64-encoded string into an abci::transaction::Hash
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
        let decoded = base64::decode(s).map_err(serde::de::Error::custom)?;
        let hash =
            Hash::from_bytes(Algorithm::Sha256, &decoded).map_err(serde::de::Error::custom)?;
        Ok(hash)
    }

    /// Serialize from an abci::transaction::Hash into a base64-encoded string
    pub fn serialize<S>(value: &Hash, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base64_bytes = base64::encode(value.as_bytes());
        let base64_string = String::from_utf8(base64_bytes).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&base64_string)
    }
}
