use candid::{encode_one, CandidType};
use serde::{Deserialize, Serialize};

/// Deployment update sent to the client via IC WebSocket
#[derive(CandidType, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum DeploymentUpdate {
    Initialized,
    DeploymentCreated(String, u64),
    LeaseCreated(String),
}

impl DeploymentUpdate {
    pub fn candid_serialize(&self) -> Vec<u8> {
        encode_one(&self).unwrap()
    }
}
