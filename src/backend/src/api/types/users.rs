use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};

#[derive(Debug, CandidType, Deserialize, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct UserId(Principal);

impl UserId {
    pub fn principal(&self) -> Principal {
        self.0
    }

    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<UserId> for Principal {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<Principal> for UserId {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl Storable for UserId {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum User {
    Admin,
    Deployer,
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
