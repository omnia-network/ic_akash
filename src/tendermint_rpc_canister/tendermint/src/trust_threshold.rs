//! Define traits and instances for dealing with trust thresholds.

use core::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{error::Error, prelude::*, serializers};

/// TrustThreshold defines how much of the total voting power of a known
/// and trusted validator set is sufficient for a commit to be
/// accepted going forward.
pub trait TrustThreshold: Copy + Clone + Debug + Serialize + DeserializeOwned {
    /// Check whether the given signed voting power is sufficient according to
    /// this trust threshold against the given total voting power.
    fn is_enough_power(&self, signed_voting_power: u64, total_voting_power: u64) -> bool;
}

/// TrustThresholdFraction defines what fraction of the total voting power of a known
/// and trusted validator set is sufficient for a commit to be
/// accepted going forward.
/// The [`Default::default()`] returns true, iff at least a third of the trusted
/// voting power signed (in other words at least one honest validator signed).
/// Some clients might require more than +1/3 and can implement their own
/// [`TrustThreshold`] which can be passed into all relevant methods.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    try_from = "RawTrustThresholdFraction",
    into = "RawTrustThresholdFraction"
)]
pub struct TrustThresholdFraction {
    numerator: u64,
    denominator: u64,
}

impl TrustThresholdFraction {
    /// Constant for a trust threshold of 1/3.
    pub const ONE_THIRD: Self = Self {
        numerator: 1,
        denominator: 3,
    };

    /// Constant for a trust threshold of 2/3.
    pub const TWO_THIRDS: Self = Self {
        numerator: 2,
        denominator: 3,
    };

    /// Instantiate a TrustThresholdFraction if the given denominator and
    /// numerator are valid.
    ///
    /// The parameters are valid iff `1/3 <= numerator/denominator <= 1`.
    /// In any other case we return an error.
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, Error> {
        if numerator > denominator {
            return Err(Error::trust_threshold_too_large());
        }
        if denominator == 0 {
            return Err(Error::undefined_trust_threshold());
        }
        if 3 * numerator < denominator {
            return Err(Error::trust_threshold_too_small());
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// The numerator of this fraction.
    pub fn numerator(&self) -> u64 {
        self.numerator
    }

    /// The denominator of this fraction.
    pub fn denominator(&self) -> u64 {
        self.denominator
    }
}

impl TryFrom<RawTrustThresholdFraction> for TrustThresholdFraction {
    type Error = Error;

    fn try_from(value: RawTrustThresholdFraction) -> Result<Self, Self::Error> {
        Self::new(value.numerator, value.denominator)
    }
}

impl From<TrustThresholdFraction> for RawTrustThresholdFraction {
    fn from(f: TrustThresholdFraction) -> Self {
        Self {
            numerator: f.numerator,
            denominator: f.denominator,
        }
    }
}

impl TrustThreshold for TrustThresholdFraction {
    fn is_enough_power(&self, signed_voting_power: u64, total_voting_power: u64) -> bool {
        signed_voting_power * self.denominator > total_voting_power * self.numerator
    }
}

impl Default for TrustThresholdFraction {
    fn default() -> Self {
        Self::ONE_THIRD
    }
}

impl Display for TrustThresholdFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Facilitates validation of [`TrustThresholdFraction`] instances when
/// deserializing them.
#[derive(Serialize, Deserialize)]
pub struct RawTrustThresholdFraction {
    #[serde(with = "serializers::from_str")]
    numerator: u64,
    #[serde(with = "serializers::from_str")]
    denominator: u64,
}
