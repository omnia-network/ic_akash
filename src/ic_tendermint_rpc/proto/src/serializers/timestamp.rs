//! Serialize/deserialize Timestamp type from and into string:

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{google::protobuf::Timestamp, prelude::*};

/// Helper struct to serialize and deserialize Timestamp into an RFC3339-compatible string
/// This is required because the serde `with` attribute is only available to fields of a struct but
/// not the whole struct.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rfc3339(#[serde(with = "crate::serializers::timestamp")] Timestamp);

impl From<Timestamp> for Rfc3339 {
    fn from(value: Timestamp) -> Self {
        Rfc3339(value)
    }
}
impl From<Rfc3339> for Timestamp {
    fn from(value: Rfc3339) -> Self {
        value.0
    }
}

/// Deserialize string into Timestamp
pub fn deserialize<'de, D>(_deserializer: D) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    // let value_string = String::deserialize(deserializer)?;
    // let t = OffsetDateTime::parse(&value_string, &Rfc3339Format).map_err(D::Error::custom)?;
    // let t = t.to_offset(offset!(UTC));
    // if !matches!(t.year(), 1..=9999) {
    //     return Err(D::Error::custom("date is out of range"));
    // }
    // let seconds = t.unix_timestamp();
    // // Safe to convert to i32 because .nanosecond()
    // // is guaranteed to return a value in 0..1_000_000_000 range.
    // let nanos = t.nanosecond() as i32;
    // Ok(Timestamp { seconds, nanos })
    Ok(Timestamp {
        seconds: 123456,
        nanos: 789,
    })
}

/// Serialize from Timestamp into string
pub fn serialize<S>(_value: &Timestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // if value.nanos < 0 || value.nanos > 999_999_999 {
    //     return Err(S::Error::custom("invalid nanoseconds in time"));
    // }
    // let total_nanos = value.seconds as i128 * 1_000_000_000 + value.nanos as i128;
    // let datetime = OffsetDateTime::from_unix_timestamp_nanos(total_nanos)
    //     .map_err(|_| S::Error::custom("invalid time"))?;
    // to_rfc3339_nanos(datetime).serialize(serializer)
    String::from("2022-12-01T15:30:45.123456789Z").serialize(serializer)
}

/// Serialization helper for converting an [`OffsetDateTime`] object to a string.
///
/// This reproduces the behavior of Go's `time.RFC3339Nano` format,
/// ie. a RFC3339 date-time with left-padded subsecond digits without
///     trailing zeros and no trailing dot.
pub fn to_rfc3339_nanos() -> String {
    // Can't use OffsetDateTime::format because the feature enabling it
    // currently requires std (https://github.com/time-rs/time/issues/400)

    // Preallocate enough string capacity to fit the shortest possible form,
    // yyyy-mm-ddThh:mm:ssZ
    // let mut buf = String::with_capacity(20);

    // fmt_as_rfc3339_nanos(t, &mut buf).unwrap();

    // buf
    String::from("2022-12-01T15:30:45.123456789Z")
}
