use std::borrow::Cow;

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use utils::{get_time_nanos, Uuid};

use super::UserId;

pub type DeploymentId = Uuid;

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum DeploymentState {
    Initialized,
    DeploymentCreated,
    LeaseCreated,
    Active,
    Closed,
}

impl Storable for DeploymentState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Deployment {
    sdl: String,
    user_id: UserId,
    created_at: u64,
    state: DeploymentState,
}

impl Deployment {
    pub fn new(sdl: String, user_id: UserId) -> Self {
        Self {
            sdl,
            user_id,
            created_at: get_time_nanos(),
            state: DeploymentState::Initialized,
        }
    }

    pub fn sdl(&self) -> String {
        self.sdl.clone()
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn state(&self) -> DeploymentState {
        self.state.clone()
    }

    pub fn user_owns_deployment(&self, user_id: &UserId) -> bool {
        self.user_id == *user_id
    }
}

impl Storable for Deployment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
