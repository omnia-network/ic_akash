use crate::{
    akash::{address::get_account_id_from_public_key, bids::fetch_bids, sdl::SdlV3},
    api::{
        map_deployment, services::AkashService, AccessControlService, ApiError, ApiResult,
        Deployment, DeploymentId, DeploymentState, DeploymentUpdate, DeploymentsService,
        GetDeploymentResponse, UserId,
    },
    fixtures::example_sdl,
    helpers::send_canister_update,
};
use candid::Principal;
use ic_cdk::*;
use std::time::Duration;

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
        calling_principal: Principal,
        sdl: String,
    ) -> Result<DeploymentId, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        let parsed_sdl = SdlV3::try_from_str(&sdl)
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid SDL: {}", e)))?;

        let user_id = UserId::new(calling_principal);

        let deployment_id = self
            .deployments_service
            .init_deployment(user_id, sdl)
            .await?;

        ic_cdk_timers::set_timer(Duration::from_secs(0), move || {
            ic_cdk::spawn(async move {
                if let Err(e) =
                    handle_deployment(calling_principal, parsed_sdl, deployment_id).await
                {
                    print(&format!("Error handling deployment: {:?}", e));
                }
            });
        });

        Ok(deployment_id)
    }
}

async fn handle_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<(), ApiError> {
    let mut deployment_service = DeploymentsService::default();
    let dseq = handle_create_deployment(calling_principal, parsed_sdl, deployment_id)
        .await
        .map_err(|e| {
            deployment_service
                .set_failed_deployment(deployment_id)
                .expect("Failed to set deployment to failed");
            e
        })?;

    fetch_bids_and_create_lease(calling_principal, dseq, deployment_id);

    Ok(())
}

async fn handle_create_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<u64, ApiError> {
    let akash_service = AkashService::default();
    let mut deployment_service = DeploymentsService::default();

    let (tx_hash, dseq, _manifest) = akash_service
        .create_deployment(parsed_sdl)
        .await
        .map_err(|e| ApiError::internal(&format!("Error creating deployment: {}", e)))?;

    let new_state = deployment_service
        .update_deployment(deployment_id)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    assert_eq!(new_state, DeploymentState::DeploymentCreated);

    send_canister_update(
        calling_principal,
        DeploymentUpdate::DeploymentCreated(tx_hash, dseq),
    );

    Ok(dseq)
}

fn fetch_bids_and_create_lease(
    calling_principal: Principal,
    dseq: u64,
    deployment_id: DeploymentId,
) {
    ic_cdk_timers::set_timer(Duration::from_secs(1), move || {
        ic_cdk::spawn(async move {
            let mut deployment_service = DeploymentsService::default();

            let akash_service = AkashService::default();
            let config = akash_service.get_config();
            let public_key = config.public_key().await.expect("failed to get public key");
            let account_id =
                get_account_id_from_public_key(&public_key).expect("failed to get account id");
            let rpc_url = config.tendermint_rpc_url();

            print(&format!("Fetching bids for dseq: {}", dseq));

            let bids = fetch_bids(rpc_url.clone(), &account_id, dseq)
                .await
                .expect("failed to fetch bids");

            if bids.len() > 0 {
                let (_tx_hash, deployment_url) =
                    handle_create_lease(calling_principal, dseq, deployment_id)
                        .await
                        .map_err(|e| {
                            deployment_service
                                .set_failed_deployment(deployment_id)
                                .expect("Failed to set deployment to failed");
                            e
                        })
                        .expect("failed to handle create lease");

                print(&format!("Deployment URL: {}", deployment_url));
            } else {
                fetch_bids_and_create_lease(calling_principal, dseq, deployment_id);
            }
        })
    });
}
async fn handle_create_lease(
    calling_principal: Principal,
    dseq: u64,
    deployment_id: DeploymentId,
) -> Result<(String, String), ApiError> {
    let akash_service = AkashService::default();
    let mut deployment_service = DeploymentsService::default();

    let (tx_hash, deployment_url) = akash_service
        .create_lease(dseq)
        .await
        .map_err(|e| ApiError::internal(&format!("Error creating lease: {}", e)))?;

    let new_state = deployment_service
        .update_deployment(deployment_id)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    assert_eq!(new_state, DeploymentState::LeaseCreated);

    send_canister_update(
        calling_principal,
        DeploymentUpdate::LeaseCreated(deployment_url.clone()),
    );

    Ok((tx_hash, deployment_url))
}
