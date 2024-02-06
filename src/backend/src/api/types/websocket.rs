use candid::{encode_one, CandidType};
use serde::{Deserialize, Serialize};

use super::DeploymentUpdate;

#[derive(Debug, CandidType, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentUpdateWsMessage {
    id: String,
    update: DeploymentUpdate,
}

impl DeploymentUpdateWsMessage {
    pub fn new(id: String, update: DeploymentUpdate) -> Self {
        Self { id, update }
    }

    pub fn candid_serialize(&self) -> Vec<u8> {
        encode_one(&self).unwrap()
    }
}
