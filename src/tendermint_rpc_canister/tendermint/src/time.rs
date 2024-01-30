//! Timestamps used by Tendermint blockchains

use core::{
    convert::{TryFrom, TryInto},
    ops::{Add, Sub},
    str::FromStr,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tendermint_proto::{google::protobuf::Timestamp, serializers::timestamp, Protobuf};
use time::{
    format_description::well_known::Rfc3339,
    macros::{datetime, offset},
    OffsetDateTime, PrimitiveDateTime,
};

use crate::{error::Error, prelude::*};

/// Tendermint timestamps
///
/// A `Time` value is guaranteed to represent a valid `Timestamp` as defined
/// by Google's well-known protobuf type [specification]. Conversions and
/// operations that would result in exceeding `Timestamp`'s validity
/// range return an error or `None`.
///
/// The string serialization format for `Time` is defined as an RFC 3339
/// compliant string with the optional subsecond fraction part having
/// up to 9 digits and no trailing zeros, and the UTC offset denoted by Z.
/// This reproduces the behavior of Go's `time.RFC3339Nano` format.
///
/// [specification]: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Timestamp
// For memory efficiency, the inner member is `PrimitiveDateTime`, with assumed
// UTC offset. The `assume_utc` method is used to get the operational
// `OffsetDateTime` value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "Timestamp", into = "Timestamp")]
pub struct Time(PrimitiveDateTime);

impl Protobuf<Timestamp> for Time {}

impl TryFrom<Timestamp> for Time {
    type Error = Error;

    fn try_from(value: Timestamp) -> Result<Self, Error> {
        let nanos = value
            .nanos
            .try_into()
            .map_err(|_| Error::timestamp_nanos_out_of_range())?;
        Self::from_unix_timestamp(value.seconds, nanos)
    }
}

impl From<Time> for Timestamp {
    fn from(value: Time) -> Self {
        let t = value.0.assume_utc();
        let seconds = t.unix_timestamp();
        // Safe to convert to i32 because .nanosecond()
        // is guaranteed to return a value in 0..1_000_000_000 range.
        let nanos = t.nanosecond() as i32;
        Timestamp { seconds, nanos }
    }
}

impl Time {
    #[cfg(feature = "clock")]
    pub fn now() -> Time {
        OffsetDateTime::now_utc().try_into().unwrap()
    }

    // Internal helper to produce a `Time` value validated with regard to
    // the date range allowed in protobuf timestamps.
    // The source `OffsetDateTime` value must have the zero UTC offset.
    fn from_utc(t: OffsetDateTime) -> Result<Self, Error> {
        debug_assert_eq!(t.offset(), offset!(UTC));
        match t.year() {
            1..=9999 => Ok(Self(PrimitiveDateTime::new(t.date(), t.time()))),
            _ => Err(Error::date_out_of_range()),
        }
    }

    /// Get the unix epoch ("1970-01-01 00:00:00 UTC") as a [`Time`]
    pub fn unix_epoch() -> Self {
        Self(datetime!(1970-01-01 00:00:00))
    }

    pub fn from_unix_timestamp(secs: i64, nanos: u32) -> Result<Self, Error> {
        if nanos > 999_999_999 {
            return Err(Error::timestamp_nanos_out_of_range());
        }
        let total_nanos = secs as i128 * 1_000_000_000 + nanos as i128;
        match OffsetDateTime::from_unix_timestamp_nanos(total_nanos) {
            Ok(odt) => Self::from_utc(odt),
            _ => Err(Error::timestamp_conversion()),
        }
    }

    /// Calculate the amount of time which has passed since another [`Time`]
    /// as a [`core::time::Duration`]
    pub fn duration_since(&self, other: Time) -> Result<Duration, Error> {
        let duration = self.0.assume_utc() - other.0.assume_utc();
        duration
            .try_into()
            .map_err(|_| Error::duration_out_of_range())
    }

    /// Parse [`Time`] from an RFC 3339 date
    pub fn parse_from_rfc3339(s: &str) -> Result<Self, Error> {
        let date = OffsetDateTime::parse(s, &Rfc3339)
            .map_err(Error::time_parse)?
            .to_offset(offset!(UTC));
        Self::from_utc(date)
    }

    /// Return an RFC 3339 and ISO 8601 date and time string with subseconds (if nonzero) and Z.
    pub fn to_rfc3339(&self) -> String {
        // timestamp::to_rfc3339_nanos(self.0.assume_utc())
        timestamp::to_rfc3339_nanos()
    }

    /// Return a Unix timestamp in seconds.
    pub fn unix_timestamp(&self) -> i64 {
        self.0.assume_utc().unix_timestamp()
    }

    /// Return a Unix timestamp in nanoseconds.
    pub fn unix_timestamp_nanos(&self) -> i128 {
        self.0.assume_utc().unix_timestamp_nanos()
    }

    /// Computes `self + duration`, returning `None` if an overflow occurred.
    pub fn checked_add(self, duration: Duration) -> Option<Self> {
        let duration = duration.try_into().ok()?;
        let t = self.0.checked_add(duration)?;
        Self::from_utc(t.assume_utc()).ok()
    }

    /// Computes `self - duration`, returning `None` if an overflow occurred.
    pub fn checked_sub(self, duration: Duration) -> Option<Self> {
        let duration = duration.try_into().ok()?;
        let t = self.0.checked_sub(duration)?;
        Self::from_utc(t.assume_utc()).ok()
    }

    /// Check whether this time is before the given time.
    pub fn before(&self, other: Time) -> bool {
        self.0.assume_utc() < other.0.assume_utc()
    }

    /// Check whether this time is after the given time.
    pub fn after(&self, other: Time) -> bool {
        self.0.assume_utc() > other.0.assume_utc()
    }
}

// impl fmt::Display for Time {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
//         timestamp::fmt_as_rfc3339_nanos(self.0.assume_utc(), f)
//     }
// }

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_from_rfc3339(s)
    }
}

impl TryFrom<OffsetDateTime> for Time {
    type Error = Error;

    fn try_from(t: OffsetDateTime) -> Result<Time, Error> {
        Self::from_utc(t.to_offset(offset!(UTC)))
    }
}

impl From<Time> for OffsetDateTime {
    fn from(t: Time) -> OffsetDateTime {
        t.0.assume_utc()
    }
}

impl Add<Duration> for Time {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Duration) -> Self::Output {
        let duration = rhs.try_into().map_err(|_| Error::duration_out_of_range())?;
        let t = self
            .0
            .checked_add(duration)
            .ok_or_else(Error::duration_out_of_range)?;
        Self::from_utc(t.assume_utc())
    }
}

impl Sub<Duration> for Time {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Duration) -> Self::Output {
        let duration = rhs.try_into().map_err(|_| Error::duration_out_of_range())?;
        let t = self
            .0
            .checked_sub(duration)
            .ok_or_else(Error::duration_out_of_range)?;
        Self::from_utc(t.assume_utc())
    }
}

/// Parse [`Time`] from a type
pub trait ParseTimestamp {
    /// Parse [`Time`], or return an [`Error`] if parsing failed
    fn parse_timestamp(&self) -> Result<Time, Error>;
}
