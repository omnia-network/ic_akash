use super::{TimestampNs, UserId};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utils::{get_time_nanos, Uuid};

pub type DeploymentId = Uuid;

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Deployment {
    sdl: String,
    user_id: UserId,
    state_history: Vec<(TimestampNs, DeploymentUpdate)>,
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

    pub fn get_history(&self) -> Vec<(u64, DeploymentUpdate)> {
        self.state_history.clone()
    }

    pub fn get_akash_info(&self) -> Option<u64> {
        self.state_history
            .iter()
            .filter_map(|(_, update)| update.get_akash_info())
            .collect::<Vec<u64>>()
            .first()
            .cloned()
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

/// Deployment update sent to the client via IC WebSocket
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum DeploymentUpdate {
    Initialized,
    DeploymentCreated {
        tx_hash: String,
        dseq: u64,
    },
    LeaseCreated {
        tx_hash: String,
        provider_url: String,
    },
    Opened,
    Closed,
    Failed {
        reason: String,
    },
}

impl DeploymentUpdate {
    pub fn get_akash_info(&self) -> Option<u64> {
        match self {
            DeploymentUpdate::DeploymentCreated { dseq, .. } => Some(*dseq),
            _ => None,
        }
    }
}

impl Storable for DeploymentUpdate {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
