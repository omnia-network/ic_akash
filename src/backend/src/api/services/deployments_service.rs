use crate::{
    api::{
        config_state, init_deployments, ApiError, Config, Deployment, DeploymentId,
        DeploymentParams, DeploymentState, DeploymentUpdateWsMessage, DeploymentsMemory, UserId,
    },
    helpers::{send_canister_update, uakt_to_akt},
};
use candid::Principal;

use super::{log_info, log_warn};

pub struct DeploymentsService {
    deployments_memory: DeploymentsMemory,
}

impl Default for DeploymentsService {
    fn default() -> Self {
        Self {
            deployments_memory: init_deployments(),
        }
    }
}

impl DeploymentsService {
    pub fn get_config(&self) -> Config {
        config_state(|state| state.clone())
    }

    pub fn get_deployment(&self, deployment_id: &DeploymentId) -> Result<Deployment, ApiError> {
        self.deployments_memory
            .get(deployment_id)
            .ok_or_else(|| ApiError::not_found(&format!("Deployment {} not found", deployment_id)))
    }

    pub fn get_deployments_for_user(&self, user_id: &UserId) -> Vec<(DeploymentId, Deployment)> {
        self.deployments_memory
            .iter()
            .filter(|(_, deployment)| deployment.user_owns_deployment(user_id))
            .map(|(k, v)| (k, v.clone()))
            .collect()
    }

    pub async fn init_deployment(
        &mut self,
        user_id: UserId,
        sdl_params: DeploymentParams,
        akt_price: f64,
        icp_price: f64,
    ) -> Result<DeploymentId, ApiError> {
        let deployment_id = DeploymentId::new()
            .await
            .map_err(|e| ApiError::internal(&format!("Failed to create deployment id: {}", e)))?;

        let deployment = Deployment::new(sdl_params, user_id, akt_price, icp_price);

        self.deployments_memory.insert(deployment_id, deployment);

        Ok(deployment_id)
    }

    pub fn check_deployment_state(&self, deployment_id: DeploymentId) -> Result<(), ApiError> {
        let deployment_state = self.get_deployment(&deployment_id)?.state();

        // if the deployment is not in the Active or LeaseCreated state, it cannot be closed, updated or deposited being made to it
        // either it fails while being creating deployment and lease (and thus there is no need to close it)
        // or it eventually gets to the LeaseCreated or Active state (and from that point on it can be closed, updated or deposited being made to it)
        match deployment_state {
            DeploymentState::Initialized
            | DeploymentState::DeploymentCreated { .. }
            | DeploymentState::Active
            | DeploymentState::LeaseCreated { .. } => {
                log_info!(
                    format!(
                        "[Deployment {}]: Closing deployment in {:?} state",
                        deployment_id, deployment_state
                    ),
                    "check_deployment_state"
                );

                Ok(())
            }
            DeploymentState::FailedOnCanister { .. } | DeploymentState::FailedOnClient { .. } => {
                log_warn!(
                    format!(
                        "[Deployment {}]: Deployment is already in {:?} state",
                        deployment_id, deployment_state
                    ),
                    "check_deployment_state"
                );

                Ok(())
            }
            DeploymentState::Closed => Err(ApiError::permission_denied(&format!(
                "Cannot close deployment. Current state: {:?}",
                deployment_state
            ))),
        }
    }

    pub fn set_failed_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: DeploymentId,
        reason: String,
    ) -> Result<(), ApiError> {
        self.update_deployment_state(
            calling_principal,
            deployment_id,
            DeploymentState::FailedOnCanister {
                reason: reason.clone(),
            },
            true,
        )
    }

    pub fn get_akash_deployment_info(
        &self,
        deployment_id: &DeploymentId,
    ) -> Result<Option<u64>, ApiError> {
        self.deployments_memory
            .get(deployment_id)
            .map(|deployment| deployment.get_akash_info())
            .ok_or(ApiError::not_found(&format!(
                "Deployment {} not found",
                deployment_id
            )))
    }

    pub fn set_close_deployment(
        &mut self,
        calling_principal: Principal,
        deployment_id: DeploymentId,
    ) -> Result<(), ApiError> {
        self.update_deployment_state(
            calling_principal,
            deployment_id,
            DeploymentState::Closed,
            false,
        )
    }

    pub fn update_deployment_state(
        &mut self,
        calling_principal: Principal,
        deployment_id: DeploymentId,
        deployment_update: DeploymentState,
        notify_client: bool,
    ) -> Result<(), ApiError> {
        let mut deployment = self.deployments_memory.get(&deployment_id).ok_or_else(|| {
            ApiError::not_found(&format!("Deployment {} not found", deployment_id))
        })?;

        // the deployment can still be closed if it failed on canister,
        // it may just fail closing but we let this case pass
        if matches!(deployment.state(), DeploymentState::FailedOnCanister { .. })
            && !matches!(deployment_update, DeploymentState::Closed)
        {
            return Err(ApiError::internal(&format!(
                "Deployment {} already failed",
                deployment_id
            )));
        }

        if let DeploymentState::Closed = deployment.state() {
            return Err(ApiError::internal(&format!(
                "Deployment {} already closed",
                deployment_id
            )));
        }

        deployment.update_state(deployment_update.clone());
        self.deployments_memory.insert(deployment_id, deployment);

        if notify_client {
            send_canister_update(
                calling_principal,
                DeploymentUpdateWsMessage::new(deployment_id.to_string(), deployment_update),
            );
        }

        Ok(())
    }

    // TODO: calculate price based on the deployment specs
    pub fn get_deployment_akt_price(&self) -> f64 {
        let uakt_amount = self.get_config().akash_config().min_deposit_uakt_amount;

        uakt_to_akt(uakt_amount)
    }
}
