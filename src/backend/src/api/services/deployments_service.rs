use crate::api::{
    init_deployments, ApiError, Deployment, DeploymentId, DeploymentState, DeploymentsMemory,
    UserId,
};

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
    pub fn get_deployment(&self, deployment_id: &DeploymentId) -> Result<Deployment, ApiError> {
        self.deployments_memory
            .get(deployment_id)
            .ok_or_else(|| ApiError::not_found(&format!("Deployment {} not found", deployment_id)))
    }

    pub fn get_deployments_for_user(&self, user_id: &UserId) -> Vec<(DeploymentId, Deployment)> {
        self.deployments_memory
            .iter()
            .filter(|(_, deployment)| deployment.user_owns_deployment(user_id))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub async fn init_deployment(
        &mut self,
        user_id: UserId,
        sdl: String,
    ) -> Result<DeploymentId, ApiError> {
        let deployment_id = DeploymentId::new()
            .await
            .map_err(|e| ApiError::internal(&format!("Failed to create deployment id: {}", e)))?;

        let deployment = Deployment::new(sdl, user_id);

        self.deployments_memory.insert(deployment_id, deployment);

        Ok(deployment_id)
    }

    pub fn update_deployment(
        &mut self,
        deployment_id: DeploymentId,
    ) -> Result<DeploymentState, ApiError> {
        let mut deployment = self.deployments_memory.get(&deployment_id).ok_or_else(|| {
            ApiError::internal(&format!("Deployment {} not found", deployment_id))
        })?;
        match deployment.state() {
            DeploymentState::Initialized => {
                let new_deployment_state = DeploymentState::DeploymentCreated;
                deployment.update_state(new_deployment_state.clone());
                self.deployments_memory.insert(deployment_id, deployment);
                Ok(new_deployment_state)
            }
            DeploymentState::DeploymentCreated => {
                let new_deployment_state = DeploymentState::LeaseCreated;
                deployment.update_state(new_deployment_state.clone());
                self.deployments_memory.insert(deployment_id, deployment);
                Ok(new_deployment_state)
            }
            DeploymentState::LeaseCreated => {
                let new_deployment_state = DeploymentState::Active;
                deployment.update_state(new_deployment_state.clone());
                self.deployments_memory.insert(deployment_id, deployment);
                Ok(new_deployment_state)
            }
            DeploymentState::Active => {
                let new_deployment_state = DeploymentState::Closed;
                deployment.update_state(new_deployment_state.clone());
                self.deployments_memory.insert(deployment_id, deployment);
                Ok(new_deployment_state)
            }
            DeploymentState::Closed | DeploymentState::Failed => Err(ApiError::internal(&format!(
                "Deployment {} already closed",
                deployment_id
            ))),
        }
    }

    pub fn set_failed_deployment(&mut self, deployment_id: DeploymentId) -> Result<(), ApiError> {
        let mut deployment = self.deployments_memory.get(&deployment_id).ok_or_else(|| {
            ApiError::internal(&format!("Deployment {} not found", deployment_id))
        })?;
        deployment.update_state(DeploymentState::Failed);
        self.deployments_memory.insert(deployment_id, deployment);
        Ok(())
    }
}
