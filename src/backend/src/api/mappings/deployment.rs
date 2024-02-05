use candid::{CandidType, Deserialize, Principal};

use crate::api::{Deployment, DeploymentUpdate, TimestampNs};

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct MappedDeployment {
    sdl: String,
    user_id: Principal,
    created_at: TimestampNs,
    state_history: Vec<(TimestampNs, DeploymentUpdate)>,
}

impl From<Deployment> for MappedDeployment {
    fn from(deployment: Deployment) -> Self {
        Self {
            sdl: deployment.sdl(),
            user_id: deployment.user_id().principal(),
            created_at: deployment.created_at(),
            state_history: deployment.get_history(),
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct GetDeploymentResponse {
    id: String,
    deployment: MappedDeployment,
}

pub fn map_deployment(deployment_id: String, deployment: Deployment) -> GetDeploymentResponse {
    GetDeploymentResponse {
        id: deployment_id.to_string(),
        deployment: deployment.into(),
    }
}
