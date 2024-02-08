use crate::{
    akash::{address::get_account_id_from_public_key, bids::fetch_bids, sdl::SdlV3},
    api::{
        map_deployment, services::AkashService, AccessControlService, ApiError, ApiResult,
        Deployment, DeploymentId, DeploymentUpdate, DeploymentsService, GetDeploymentResponse,
        UserId,
    },
    fixtures::example_sdl,
};
use candid::Principal;
use ic_cdk::{caller, print, query, update};
use std::time::Duration;

const POLLING_BIDS_INTERVAL: u64 = 3;
const MAX_FETCH_BIDS_RETRIES: u64 = 5;

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
                    map_deployment(deployment_id.to_string(), deployment)
                })
                .collect()
        })
        .into()
}

#[update]
async fn create_certificate(
    cert_pem_base64: String,
    pub_key_pem_base64: String,
) -> ApiResult<String> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .create_certificate(calling_principal, cert_pem_base64, pub_key_pem_base64)
        .await
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
async fn update_deployment(deployment_id: String, update: DeploymentUpdate) -> ApiResult<()> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .update_deployment(calling_principal, deployment_id, update)
        .await
        .into()
}

#[update]
async fn create_test_deployment() -> ApiResult<String> {
    let sdl = example_sdl().to_string();

    create_deployment(sdl).await
}

#[update]
async fn close_deployment(deployment_id: String) -> ApiResult {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .close_deployment(calling_principal, deployment_id)
        .await
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
            .assert_principal_owns_deployment(calling_principal, &deployment_id)?;

        self.deployments_service.get_deployment(&deployment_id)
    }

    pub fn get_deployments(
        &self,
        calling_principal: &Principal,
    ) -> Result<Vec<(DeploymentId, Deployment)>, ApiError> {
        // no need to check if the user owns the deployments as the 'get_deployments_for_user' function returns only the deployments the user owns
        self.access_control_service
            .assert_principal_is_user(calling_principal)?;

        let user_id = UserId::new(*calling_principal);

        let deployments = self.deployments_service.get_deployments_for_user(&user_id);

        Ok(deployments)
    }

    pub async fn create_certificate(
        &self,
        calling_principal: Principal,
        cert_pem_base64: String,
        pub_key_pem_base64: String,
    ) -> Result<String, ApiError> {
        self.access_control_service
            .assert_principal_is_user(&calling_principal)?;

        AkashService::default()
            .create_certificate(cert_pem_base64, pub_key_pem_base64)
            .await
            .map_err(|e| ApiError::internal(&format!("Error creating certificate: {}", e)))
    }

    pub async fn create_deployment(
        &mut self,
        calling_principal: Principal,
        sdl: String,
    ) -> Result<DeploymentId, ApiError> {
        self.access_control_service
            .assert_principal_is_user(&calling_principal)?;

        let parsed_sdl = SdlV3::try_from_str(&sdl)
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid SDL: {}", e)))?;

        let user_id = UserId::new(calling_principal);

        let deployment_id = self
            .deployments_service
            .init_deployment(user_id, sdl)
            .await?;

        print(&format!("[{:?}]: Initialized", deployment_id));

        ic_cdk_timers::set_timer(Duration::from_secs(0), move || {
            ic_cdk::spawn(async move {
                print(&format!(
                    "[{:?}]: Starting to handle deployment creation",
                    deployment_id
                ));

                if let Err(e) =
                    handle_deployment(calling_principal, parsed_sdl, deployment_id).await
                {
                    set_failed_deployment(
                        deployment_id,
                        calling_principal,
                        format!("Error handling deployment: {:?}", e),
                        true,
                    )
                    .await;
                }
            });
        });

        Ok(deployment_id)
    }

    pub async fn update_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: String,
        update: DeploymentUpdate,
    ) -> Result<(), ApiError> {
        let deployment_id = DeploymentId::try_from(&deployment_id[..])
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_principal_owns_deployment(&calling_principal, &deployment_id)?;

        // the client can only update the deployment state only after a lease has been created
        match self
            .deployments_service
            .get_deployment(&deployment_id)?
            .state()
        {
            DeploymentUpdate::LeaseCreated { .. } => {}
            _ => {
                return Err(ApiError::invalid_argument(&format!(
                    "Deployment must be in LeaseCreated state"
                )))
            }
        }

        match update {
            DeploymentUpdate::Active => self.deployments_service.update_deployment(
                calling_principal,
                deployment_id,
                update,
                false,
            ),
            DeploymentUpdate::FailedOnClient { .. } => {
                self.deployments_service.update_deployment(
                    calling_principal,
                    deployment_id,
                    update,
                    false,
                )?;

                try_close_akash_deployment(&deployment_id).await?;

                Ok(())
            }
            _ => Err(ApiError::invalid_argument(&format!(
                "Invalid update for deployment: {:?}",
                update
            ))),
        }
    }

    pub async fn close_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: String,
    ) -> Result<(), ApiError> {
        let deployment_id = DeploymentId::try_from(&deployment_id[..])
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_principal_owns_deployment(&calling_principal, &deployment_id)?;

        let deployment_state = DeploymentsService::default()
            .get_deployment(&deployment_id)?
            .state();
        // if the deployment is not in the Active or LeaseCreated state, it cannot be closed
        // either it fails while being creating deployment and lease (and thus there is no need to close it)
        // or it eventually gets to the LeaseCreated or Activestate (and from that point on it can be closed)
        match deployment_state {
            DeploymentUpdate::Active | DeploymentUpdate::LeaseCreated { .. } => {
        if let Err(e) = handle_close_deployment(deployment_id).await {
                    set_failed_deployment(
                deployment_id,
                calling_principal,
                format!("Error closing deployment: {:?}", e),
                // the failure happened while closing the deployment, so there is no need to do it again
                false,
            )
            .await;
                } else {
                    print(&format!("[{:?}]: Closed", deployment_id));
                }
        Ok(())
            }
            _ => Err(ApiError::internal(&format!(
                "Deployment is not active. Cannot close it now. Current state: {:?}",
                deployment_state
            ))),
        }
    }
}

async fn handle_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<(), ApiError> {
    let (_tx_hash, dseq) =
        handle_create_deployment(calling_principal, parsed_sdl, deployment_id).await?;

    print(&format!(
        "[{:?}]: Starting to handle lease creation",
        deployment_id
    ));
    handle_lease(calling_principal, dseq, deployment_id, 0);

    Ok(())
}

async fn handle_create_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<(String, u64), ApiError> {
    let akash_service = AkashService::default();
    let mut deployment_service = DeploymentsService::default();

    let (tx_hash, dseq, _manifest) = akash_service
        .create_deployment(parsed_sdl)
        .await
        .map_err(|e| ApiError::internal(&format!("Error creating deployment: {}", e)))?;

    let deployment_update = DeploymentUpdate::DeploymentCreated {
        tx_hash: tx_hash.clone(),
        dseq,
    };

    deployment_service
        .update_deployment(calling_principal, deployment_id, deployment_update, true)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    print(&format!("[{:?}]: Created deployment", deployment_id));

    Ok((tx_hash, dseq))
}

fn handle_lease(calling_principal: Principal, dseq: u64, deployment_id: DeploymentId, retry: u64) {
    ic_cdk_timers::set_timer(Duration::from_secs(POLLING_BIDS_INTERVAL), move || {
        ic_cdk::spawn(async move {
            match try_fetch_bids_and_create_lease(calling_principal, dseq, deployment_id).await {
                Ok(Some((_tx_hash, deployment_url))) => {
                    print(&format!("[{:?}]: Lease created", deployment_id));
                    print(&format!(
                        "[{:?}]: Deployment URL: {}",
                        deployment_id, deployment_url
                    ));
                }
                Ok(None) => {
                    if retry > MAX_FETCH_BIDS_RETRIES {
                        print(&format!(
                            "[{:?}]: Too many retries fetching bids",
                            deployment_id
                        ));
                        set_failed_deployment(
                            deployment_id,
                            calling_principal,
                            String::from("No bids found"),
                            true,
                        )
                        .await;
                    } else {
                        handle_lease(calling_principal, dseq, deployment_id, retry + 1);
                    }
                }
                Err(e) => {
                    set_failed_deployment(
                        deployment_id,
                        calling_principal,
                        format!("Error fetching bids and creating lease: {:?}", e),
                        true,
                    )
                    .await;
                }
            }
        })
    });
}

async fn try_fetch_bids_and_create_lease(
    calling_principal: Principal,
    dseq: u64,
    deployment_id: DeploymentId,
) -> Result<Option<(String, String)>, ApiError> {
    // if the deployment has failed, there is no need to keep fetching bids
    if let DeploymentUpdate::FailedOnCanister { .. } = DeploymentsService::default()
        .get_deployment(&deployment_id)?
        .state()
    {
        return Err(ApiError::internal(&format!(
            "Deployment failed. Stopped fetching bids"
        )));
    }

    // if the deployment is closed, there is no need to keep fetching bids
    if let DeploymentUpdate::Closed = DeploymentsService::default()
        .get_deployment(&deployment_id)?
        .state()
    {
        return Err(ApiError::internal(&format!(
            "Deployment closed. Stopped fetching bids"
        )));
    }

    let akash_service = AkashService::default();
    let config = akash_service.get_config();
    let public_key = config
        .public_key()
        .await
        .map_err(|e| ApiError::internal(&format!("failed to get public key: {}", e)))?;
    let account_id = get_account_id_from_public_key(&public_key)
        .map_err(|e| ApiError::internal(&format!("failed to get account id: {}", e)))?;
    let rpc_url = config.tendermint_rpc_url();

    print(&format!(
        "[{:?}]: Fetching bids for dseq: {}",
        deployment_id, dseq
    ));

    let bids = fetch_bids(rpc_url.clone(), &account_id, dseq)
        .await
        .expect("failed to fetch bids");

    if bids.len() > 0 {
        print(&format!("[{:?}]: Bids found", deployment_id));
        let (tx_hash, deployment_url) =
            handle_create_lease(calling_principal, dseq, deployment_id).await?;
        return Ok(Some((tx_hash, deployment_url)));
    }
    Ok(None)
}

async fn handle_create_lease(
    calling_principal: Principal,
    dseq: u64,
    deployment_id: DeploymentId,
) -> Result<(String, String), ApiError> {
    let akash_service = AkashService::default();
    let mut deployment_service = DeploymentsService::default();

    let (tx_hash, provider_url) = akash_service
        .create_lease(dseq)
        .await
        .map_err(|e| ApiError::internal(&format!("Error creating lease: {}", e)))?;

    let deployment_update = DeploymentUpdate::LeaseCreated {
        tx_hash: tx_hash.clone(),
        provider_url: provider_url.clone(),
    };
    deployment_service
        .update_deployment(calling_principal, deployment_id, deployment_update, true)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    print(&format!("[{:?}]: Lease created", deployment_id));

    Ok((tx_hash, provider_url))
}

async fn handle_close_deployment(deployment_id: DeploymentId) -> Result<(), ApiError> {
    try_close_akash_deployment(&deployment_id).await?;

    DeploymentsService::default().close_deployment(deployment_id)?;

    Ok(())
}

async fn set_failed_deployment(
    deployment_id: DeploymentId,
    calling_principal: Principal,
    reason: String,
    // determines whether the Akash deployment should be closed
    and_close: bool,
) {
    if and_close {
        let _ = try_close_akash_deployment(&deployment_id).await;
    }

    if let Err(e) = DeploymentsService::default().set_failed_deployment(
        calling_principal,
        deployment_id,
        reason.clone(),
    ) {
        print(&format!(
            "[{:?}]: Failed to set deployment as failed: {:?}",
            deployment_id, e
        ));
    } else {
        print(&format!(
            "[{:?}]: Set deployment as failed: {}",
            deployment_id, reason
        ));
    }
}

async fn try_close_akash_deployment(deployment_id: &DeploymentId) -> Result<(), ApiError> {
    print(&format!(
        "[{:?}]: Starting to close Akash deployment",
        deployment_id
    ));

    let dseq = DeploymentsService::default()
        .get_akash_deployment_info(&deployment_id)?
        .ok_or(ApiError::not_found(&format!(
            "Deployment {:?} is initialized but has not been created",
            deployment_id
        )))?;

    AkashService::default()
        .close_deployment(dseq)
        .await
        .map_err(|e| ApiError::internal(&format!("Error closing Akash deployment: {}", e)))?;

    print(&format!("[{:?}]: Closed Akash deployment", deployment_id));

    Ok(())
}
