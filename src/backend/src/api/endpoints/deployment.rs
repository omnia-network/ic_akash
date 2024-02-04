use candid::Principal;
use ic_cdk::*;
use ic_websocket_cdk::ClientPrincipal;

use crate::api::{
    map_deployment, AccessControlService, ApiError, ApiResult, AppMessage, Deployment,
    DeploymentId, DeploymentsService, GetDeploymentResponse, UserId,
};

#[query]
fn get_deployment(deployment_id: String) -> ApiResult<GetDeploymentResponse> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .get_deployment(&calling_principal, &deployment_id)
        .map(|deployment| map_deployment(deployment_id, deployment))
        .into()
}

#[query]
fn get_deployments() -> ApiResult<Vec<GetDeploymentResponse>> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .get_deployments(&calling_principal)
        .map(|deployments| {
            deployments
                .into_iter()
                .map(|(deployment_id, deployment)| {
                    map_deployment(deployment_id.to_string(), deployment.clone())
                })
                .collect()
        })
        .into()
}

#[update]
async fn create_deployment(sdl: String) -> ApiResult<String> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .create_deployment(&calling_principal, sdl)
        .await
        .map(|id| id.to_string())
        .into()
}

struct DeploymentsEndpoints {
    deployments_service: DeploymentsService,
    access_control_service: AccessControlService,
}

impl Default for DeploymentsEndpoints {
    fn default() -> Self {
        Self {
            deployments_service: DeploymentsService::default(),
            access_control_service: AccessControlService::default(),
        }
    }
}

impl DeploymentsEndpoints {
    pub fn get_deployment(
        &self,
        calling_principal: &Principal,
        deployment_id: &str,
    ) -> Result<Deployment, ApiError> {
        let deployment_id = DeploymentId::try_from(deployment_id)
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_user_owns_deployment(calling_principal, &deployment_id)?;

        self.deployments_service.get_deployment(&deployment_id)
    }

    pub fn get_deployments(
        &self,
        calling_principal: &Principal,
    ) -> Result<Vec<(DeploymentId, Deployment)>, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(calling_principal)?;

        let user_id = UserId::new(*calling_principal);

        let deployments = self.deployments_service.get_deployments_for_user(&user_id);

        Ok(deployments)
    }

    pub async fn create_deployment(
        &mut self,
        calling_principal: &Principal,
        sdl: String,
    ) -> Result<DeploymentId, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(calling_principal)?;

        let user_id = UserId::new(*calling_principal);

        let deployment = Deployment::new(sdl, user_id);

        self.deployments_service.create_deployment(deployment).await
    }
}
