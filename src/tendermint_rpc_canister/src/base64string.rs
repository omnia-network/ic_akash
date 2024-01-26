/// Serialize into base64string, deserialize from base64string
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
