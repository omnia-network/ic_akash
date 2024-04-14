use candid::{CandidType, Deserialize, Principal};

use crate::api::{Deployment, DeploymentParams, DeploymentState, TimestampNs};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct MappedDeployment {
    sdl: DeploymentParams,
    user_id: Principal,
    state_history: Vec<(TimestampNs, DeploymentState)>,
    icp_price: f64,
}

impl From<Deployment> for MappedDeployment {
    fn from(deployment: Deployment) -> Self {
        Self {
            sdl: deployment.sdl(),
            user_id: deployment.user_id().principal(),
            state_history: deployment.get_history(),
            icp_price: deployment.icp_price(),
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct GetDeploymentResponse {
    id: String,
    deployment: MappedDeployment,
}

pub fn map_deployment(deployment_id: String, deployment: Deployment) -> GetDeploymentResponse {
    GetDeploymentResponse {
        id: deployment_id,
        deployment: deployment.into(),
    }
}
