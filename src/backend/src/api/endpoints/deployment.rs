use crate::{
    akash::{address::get_account_id_from_public_key, bids::fetch_bids, sdl::SdlV3},
    api::{
        log_info, map_deployment, services::AkashService, AccessControlService, ApiError,
        ApiResult, CpuSize, Deployment, DeploymentId, DeploymentParams, DeploymentParamsPort,
        DeploymentState, DeploymentsService, GetDeploymentResponse, LedgerService, LogService,
        MemorySize, StorageSize, UserId, UsersService,
    },
    fixtures::updated_example_sdl,
    helpers::uakt_to_akt,
};
use candid::Principal;
use ic_cdk::{caller, query, update};
use std::time::Duration;

const POLLING_BIDS_INTERVAL_SECS: u64 = 3;
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
async fn create_deployment(sdl_params: DeploymentParams) -> ApiResult<String> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .create_deployment(calling_principal, sdl_params)
        .await
        .map(|id| id.to_string())
        .into()
}

#[update]
async fn update_deployment_state(deployment_id: String, update: DeploymentState) -> ApiResult<()> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .update_deployment_state(calling_principal, deployment_id, update)
        .await
        .into()
}

#[update]
async fn create_test_deployment() -> ApiResult<String> {
    let calling_principal = caller();
    let sdl_params = DeploymentParams::builder(
        "IC WebSocket Gateway".to_string(),
        "omniadevs/ic-websocket-gateway:v1.3.2".to_string(),
    )
    .cpu(CpuSize::Small)
    .memory(MemorySize::Small)
    .storage(StorageSize::Small)
    .port(DeploymentParamsPort::new(8080, 80).with_domain("akash-gateway.icws.io".to_string()))
    .command(vec![
        "/ic-ws-gateway/ic_websocket_gateway".to_string(),
        "--gateway-address".to_string(),
        "0.0.0.0:8080".to_string(),
        "--ic-network-url".to_string(),
        "https://icp-api.io".to_string(),
        "--polling-interval".to_string(),
        "400".to_string(),
    ])
    .build();

    DeploymentsEndpoints::default()
        .create_deployment(calling_principal, sdl_params)
        .await
        .map(|id| id.to_string())
        .into()
}

#[update]
async fn deposit_deployment(deployment_id: String, amount_uakt: u64) -> ApiResult<()> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .deposit_deployment(calling_principal, deployment_id, amount_uakt)
        .await
        .into()
}

#[update]
async fn update_test_deployment_sdl(deployment_id: String) -> ApiResult<()> {
    let calling_principal = caller();
    let sdl = updated_example_sdl().to_string();

    DeploymentsEndpoints::default()
        .update_deployment_sdl(calling_principal, deployment_id, sdl)
        .await
        .into()
}

#[update]
async fn close_deployment(deployment_id: String) -> ApiResult<()> {
    let calling_principal = caller();

    DeploymentsEndpoints::default()
        .close_deployment(calling_principal, deployment_id)
        .await
        .into()
}

#[update]
async fn get_deployment_icp_price() -> ApiResult<f64> {
    DeploymentsEndpoints::default()
        .get_deployment_icp_price()
        .await
        .into()
}

#[derive(Default)]
struct DeploymentsEndpoints {
    deployments_service: DeploymentsService,
    access_control_service: AccessControlService,
    log_service: LogService,
    users_service: UsersService,
    akash_service: AkashService,
    ledger_service: LedgerService,
}

impl DeploymentsEndpoints {
    fn get_deployment(
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

    fn get_deployments(
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

    async fn create_certificate(
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

    async fn create_deployment(
        &mut self,
        calling_principal: Principal,
        sdl_params: DeploymentParams,
    ) -> Result<DeploymentId, ApiError> {
        self.access_control_service
            .assert_principal_is_user(&calling_principal)?;

        let deployment_akt_price = self.deployments_service.get_deployment_akt_price();
        let deployment_icp_price = self.get_deployment_icp_price().await?;

        let canister_balance = self
            .akash_service
            .uakt_balance()
            .await
            .map_err(|e| ApiError::internal(&format!("could not get balance: {}", e)))?;
        let akt_canister_balance = uakt_to_akt(canister_balance);
        if akt_canister_balance < deployment_akt_price {
            return Err(ApiError::internal(&format!(
                "Insufficient AKT balance on canister. Remaining balance: {} AKT",
                akt_canister_balance
            )));
        }

        let parsed_sdl = SdlV3::try_from_deployment_params(sdl_params.clone())
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid SDL: {}", e)))?;

        let user_id = UserId::new(calling_principal);
        // deduct AKT from user's balance for deployment escrow
        self.users_service
            .charge_user(user_id, deployment_akt_price)?;

        let deployment_id = self
            .deployments_service
            .init_deployment(
                user_id,
                sdl_params,
                deployment_akt_price,
                deployment_icp_price,
            )
            .await?;

        self.log_service.log_info(
            format!("[Deployment {}]: Initialized", deployment_id),
            Some("create_deployment".to_string()),
        )?;

        ic_cdk_timers::set_timer(Duration::ZERO, move || {
            ic_cdk::spawn(async move {
                log_info!(
                    format!(
                        "[Deployment {}]: Starting to handle deployment creation",
                        deployment_id
                    ),
                    "create_deployment_task"
                );

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

    async fn deposit_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: String,
        amount_uakt: u64,
    ) -> Result<(), ApiError> {
        let deployment_id = DeploymentId::try_from(&deployment_id[..])
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_principal_owns_deployment(&calling_principal, &deployment_id)?;

        // TODO: check if deployment is not closed or failed

        let dseq = self
            .deployments_service
            .get_akash_deployment_info(&deployment_id)?
            .ok_or(ApiError::not_found(&format!(
                "Deployment {} is initialized but has not been created",
                deployment_id
            )))?;

        self.akash_service
            .deposit_deployment(dseq, amount_uakt)
            .await
            .map_err(|e| ApiError::internal(&format!("Error updating deployment: {}", e)))?;

        let user_id = UserId::new(calling_principal);
        // deduct AKT from user's balance for deposit to deployment escrow
        self.users_service
            .charge_user(user_id, uakt_to_akt(amount_uakt))?;

        self.log_service.log_info(
            format!("[Deployment {}]: Deposit deployment", deployment_id),
            None,
        )?;
        Ok(())
    }

    async fn update_deployment_sdl(
        &self,
        calling_principal: Principal,
        deployment_id: String,
        sdl: String,
    ) -> Result<(), ApiError> {
        let deployment_id = DeploymentId::try_from(&deployment_id[..])
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_principal_owns_deployment(&calling_principal, &deployment_id)?;

        let parsed_sdl = SdlV3::try_from_str(&sdl)
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid SDL: {}", e)))?;

        // no need to deduct AKT from user's balance for udating deplyment as the needed tokens are taken from the escrow contract

        let dseq = self
            .deployments_service
            .get_akash_deployment_info(&deployment_id)?
            .ok_or(ApiError::not_found(&format!(
                "Deployment with id {} is initialized but has not been created",
                deployment_id
            )))?;

        self.akash_service
            .update_deployment_sdl(dseq, parsed_sdl)
            .await
            .map_err(|e| ApiError::internal(&format!("Error updating deployment: {}", e)))?;

        self.log_service.log_info(
            format!("[Deployment {}]: Updated deployment", deployment_id),
            None,
        )?;
        Ok(())
    }

    async fn update_deployment_state(
        &mut self,
        calling_principal: Principal,
        deployment_id: String,
        update: DeploymentState,
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
            DeploymentState::LeaseCreated { .. } => {}
            _ => {
                return Err(ApiError::invalid_argument(
                    "Deployment must be in LeaseCreated state",
                ))
            }
        }

        match update {
            DeploymentState::Active => self.deployments_service.update_deployment_state(
                calling_principal,
                deployment_id,
                update,
                false,
            ),
            DeploymentState::FailedOnClient { .. } => {
                self.deployments_service.update_deployment_state(
                    calling_principal,
                    deployment_id,
                    update,
                    false,
                )?;

                try_close_akash_deployment(&deployment_id).await
            }
            _ => Err(ApiError::invalid_argument(&format!(
                "Invalid update for deployment: {:?}",
                update
            ))),
        }
    }

    async fn close_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: String,
    ) -> Result<(), ApiError> {
        let deployment_id = DeploymentId::try_from(&deployment_id[..])
            .map_err(|e| ApiError::invalid_argument(&format!("Invalid deployment id: {}", e)))?;

        self.access_control_service
            .assert_principal_owns_deployment(&calling_principal, &deployment_id)?;

        self.deployments_service
            .check_deployment_state(deployment_id)?;

        if let Err(e) = handle_close_deployment(calling_principal, deployment_id).await {
            set_failed_deployment(
                deployment_id,
                calling_principal,
                format!("Error closing deployment: {:?}", e),
                // the failure happened while closing the deployment, so there is no need to do it again
                false,
            )
            .await;
        } else {
            self.log_service
                .log_info(format!("[Deployment {}]: Closed", deployment_id), None)?;
            // TODO: add back to the user's balance the AKT tokens remaining from the escrow
        }
        Ok(())
    }

    async fn get_deployment_icp_price(&self) -> Result<f64, ApiError> {
        let icp_2_akt = self.ledger_service.get_icp_2_akt_conversion_rate().await?;
        let deployment_akt_price = self.deployments_service.get_deployment_akt_price();

        Ok(deployment_akt_price / icp_2_akt)
    }
}

async fn handle_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<(), ApiError> {
    let dseq = handle_create_deployment(calling_principal, parsed_sdl, deployment_id).await?;

    log_info!(
        format!(
            "[Deployment {}]: Starting to handle lease creation",
            deployment_id
        ),
        "handle_deployment"
    );
    handle_lease(calling_principal, dseq, deployment_id, 0);

    Ok(())
}

async fn handle_create_deployment(
    calling_principal: Principal,
    parsed_sdl: SdlV3,
    deployment_id: DeploymentId,
) -> Result<u64, ApiError> {
    let akash_service = AkashService::default();
    let mut deployment_service = DeploymentsService::default();

    let (tx_hash, dseq, manifest) = akash_service
        .create_deployment(parsed_sdl.clone())
        .await
        .map_err(|e| ApiError::internal(&format!("Error creating deployment: {}", e)))?;

    let deployment_update = DeploymentState::DeploymentCreated {
        tx_hash: tx_hash.clone(),
        dseq,
        manifest_sorted_json: manifest.clone(),
    };

    deployment_service
        .update_deployment_state(calling_principal, deployment_id, deployment_update, true)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    let sdl_yaml = parsed_sdl
        .to_yaml()
        .map_err(|e| ApiError::internal(&format!("Error serializing SDL to YAML: {}", e)))?;

    log_info!(
        format!(
            "[Deployment {}]: Created deployment. YAML: {}, with manifest: {}",
            deployment_id, sdl_yaml, manifest,
        ),
        "handle_create_deployment"
    );

    Ok(dseq)
}

fn handle_lease(calling_principal: Principal, dseq: u64, deployment_id: DeploymentId, retry: u64) {
    ic_cdk_timers::set_timer(Duration::from_secs(POLLING_BIDS_INTERVAL_SECS), move || {
        ic_cdk::spawn(async move {
            match try_fetch_bids_and_create_lease(calling_principal, dseq, deployment_id).await {
                Ok(Some((_tx_hash, deployment_url))) => {
                    log_info!(
                        format!(
                            "[Deployment {}]: Deployment URL: {}",
                            deployment_id, deployment_url
                        ),
                        "handle_lease"
                    );
                }
                Ok(None) => {
                    if retry > MAX_FETCH_BIDS_RETRIES {
                        log_info!(
                            format!(
                                "[Deployment {}]: Too many retries fetching bids",
                                deployment_id
                            ),
                            "handle_lease"
                        );
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
    if let DeploymentState::FailedOnCanister { .. } = DeploymentsService::default()
        .get_deployment(&deployment_id)?
        .state()
    {
        return Err(ApiError::internal(
            "Deployment failed. Stopped fetching bids",
        ));
    }

    // if the deployment is closed, there is no need to keep fetching bids
    if let DeploymentState::Closed = DeploymentsService::default()
        .get_deployment(&deployment_id)?
        .state()
    {
        return Err(ApiError::internal(
            "Deployment closed. Stopped fetching bids",
        ));
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

    log_info!(
        format!(
            "[Deployment {}]: Fetching bids for dseq: {}",
            deployment_id, dseq
        ),
        "try_fetch_bids_and_create_lease"
    );

    let bids = fetch_bids(rpc_url.clone(), &account_id, dseq)
        .await
        .map_err(|e| ApiError::internal(e.as_str()))?;

    if !bids.is_empty() {
        log_info!(
            format!("[Deployment {}]: Bids found", deployment_id),
            "try_fetch_bids_and_create_lease"
        );
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

    let deployment_update = DeploymentState::LeaseCreated {
        tx_hash: tx_hash.clone(),
        provider_url: provider_url.clone(),
    };
    deployment_service
        .update_deployment_state(calling_principal, deployment_id, deployment_update, true)
        .map_err(|e| ApiError::internal(&format!("Error updating deployment: {:?}", e)))?;

    log_info!(
        format!("[Deployment {}]: Lease created", deployment_id),
        "handle_create_lease"
    );

    Ok((tx_hash, provider_url))
}

async fn handle_close_deployment(
    calling_principal: Principal,
    deployment_id: DeploymentId,
) -> Result<(), ApiError> {
    try_close_akash_deployment(&deployment_id).await?;

    DeploymentsService::default().set_close_deployment(calling_principal, deployment_id)?;

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
        log_info!(
            format!(
                "[Deployment {}]: Failed to set deployment as failed: {:?}",
                deployment_id, e
            ),
            "set_failed_deployment"
        );
    } else {
        log_info!(
            format!(
                "[Deployment {}]: Set deployment as failed: {}",
                deployment_id, reason
            ),
            "set_failed_deployment"
        );
    }
}

async fn try_close_akash_deployment(deployment_id: &DeploymentId) -> Result<(), ApiError> {
    log_info!(
        format!(
            "[Deployment {}]: Starting to close Akash deployment",
            deployment_id
        ),
        "try_close_akash_deployment"
    );

    let dseq = DeploymentsService::default()
        .get_akash_deployment_info(deployment_id)?
        .ok_or(ApiError::not_found(&format!(
            "Deployment with id {} is initialized but has not been created",
            deployment_id
        )))?;

    AkashService::default()
        .close_deployment(dseq)
        .await
        .map_err(|e| ApiError::internal(&format!("Error closing Akash deployment: {}", e)))?;

    log_info!(
        format!("[Deployment {}]: Closed Akash deployment", deployment_id),
        "try_close_akash_deployment"
    );

    Ok(())
}
