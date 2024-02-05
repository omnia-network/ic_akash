use candid::{encode_one, CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Deployment update sent to the client via IC WebSocket
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum DeploymentUpdate {
    Initialized,
    DeploymentCreated(String, u64),
    LeaseCreated(String),
    Opened,
    Closed,
    Failed,
}

impl DeploymentUpdate {
    pub fn candid_serialize(&self) -> Vec<u8> {
        encode_one(&self).unwrap()
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
