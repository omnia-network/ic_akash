use candid::{CandidType, Deserialize, Principal};

use crate::api::{Deployment, DeploymentState, TimestampNs};

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct MappedDeployment {
    sdl: String,
    user_id: Principal,
    created_at: TimestampNs,
    state: DeploymentState,
}

impl From<Deployment> for MappedDeployment {
    fn from(deployment: Deployment) -> Self {
        Self {
            sdl: deployment.sdl(),
            user_id: deployment.user_id().principal(),
            created_at: deployment.created_at(),
            state: deployment.state(),
        }
    }
}
