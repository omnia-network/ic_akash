use std::{borrow::Cow, fmt::Display};

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use utils::get_time_nanos;

use super::TimestampNs;

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
pub enum UserRole {
    Admin,
    Deployer,
}

impl Storable for UserRole {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    role: UserRole,
    created_at: TimestampNs,
}

impl User {
    pub fn new(role: UserRole) -> Self {
        Self {
            role,
            created_at: get_time_nanos(),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn set_role(&mut self, role: UserRole) {
        self.role = role
    }
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
