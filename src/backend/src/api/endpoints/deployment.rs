use std::time::Duration;

use crate::{
    akash::sdl::SdlV3,
    api::{
        map_deployment, services::AkashService, AccessControlService, ApiError, ApiResult,
        Deployment, DeploymentId, DeploymentUpdate, DeploymentsService, GetDeploymentResponse,
        UserId,
    },
    fixtures::example_sdl,
    helpers::send_canister_update,
};
use candid::Principal;
use ic_cdk::*;

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
        .create_deployment(calling_principal, sdl)
        .await
        .map(|id| id.to_string())
        .into()
}

#[update]
async fn create_test_deployment() -> ApiResult<String> {
    let sdl = example_sdl().to_string();

    create_deployment(sdl).await
}

struct DeploymentsEndpoints {
    deployments_service: DeploymentsService,
    access_control_service: AccessControlService,
    akash_service: AkashService,
}

impl Default for DeploymentsEndpoints {
    fn default() -> Self {
        Self {
            deployments_service: DeploymentsService::default(),
            access_control_service: AccessControlService::default(),
            akash_service: AkashService::default(),
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
        calling_principal: Principal,
        sdl: String,
    ) -> Result<DeploymentId, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        let parsed_sdl = SdlV3::try_from_str(&sdl)
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid SDL: {}", e)))?;

        let user_id = UserId::new(calling_principal);

        let deployment = Deployment::new(sdl, user_id);

        let deployment_id = self
            .deployments_service
            .insert_deployment(deployment)
            .await?;

        ic_cdk_timers::set_timer(Duration::from_secs(0), move || {
            ic_cdk::spawn(async move {
                let akash_service = AkashService::default();
                let deployment_service = DeploymentsService::default();
                match akash_service.create_deployment(parsed_sdl).await {
                    Ok((tx_hash, dseq, manifest)) => {
                        print(&format!(
                            "[create_deployment] tx_hash: {}, dseq: {}, manifest: {}",
                            tx_hash, dseq, manifest
                        ));

                        send_canister_update(
                            calling_principal,
                            DeploymentUpdate::Created(tx_hash, dseq),
                        );
                    }
                    Err(err) => {
                        print(&format!("Error creating deployment: {}", err));
                    }
                }
            });
        });

        Ok(deployment_id)
    }
}
