use super::{DeploymentUpdate, UserId};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use std::borrow::Cow;
use utils::{get_time_nanos, Uuid};

pub type DeploymentId = Uuid;

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Deployment {
    sdl: String,
    user_id: UserId,
    state_history: Vec<(u64, DeploymentUpdate)>,
}

impl Deployment {
    pub fn new(sdl: String, user_id: UserId) -> Self {
        Self {
            sdl,
            user_id,
            state_history: vec![(get_time_nanos(), DeploymentUpdate::Initialized)],
        }
    }

    pub fn sdl(&self) -> String {
        self.sdl.clone()
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn created_at(&self) -> u64 {
        self.state_history.first().expect("must be initialized").0
    }

    pub fn state(&self) -> DeploymentUpdate {
        self.state_history
            .last()
            .expect("must have at least one state")
            .1
            .clone()
    }

    pub fn user_owns_deployment(&self, user_id: &UserId) -> bool {
        self.user_id == *user_id
    }

    pub fn update_state(&mut self, update: DeploymentUpdate) {
        self.state_history.push((get_time_nanos(), update));
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
