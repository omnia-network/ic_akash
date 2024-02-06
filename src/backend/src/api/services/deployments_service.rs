use crate::api::{
    init_deployments, ApiError, Deployment, DeploymentId, DeploymentUpdate, DeploymentsMemory,
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

        let deployment = Deployment::new(sdl, user_id, deployment_id.to_string());

        self.deployments_memory.insert(deployment_id, deployment);

        Ok(deployment_id)
    }

    pub fn update_deployment(
        &mut self,
        deployment_id: DeploymentId,
        deployment_update: DeploymentUpdate,
    ) -> Result<(), ApiError> {
        self.update_deployment_state(deployment_id, deployment_update)
    }

    pub fn set_failed_deployment(
        &mut self,
        deployment_id: DeploymentId,
        reason: String,
    ) -> Result<(), ApiError> {
        self.update_deployment_state(deployment_id, DeploymentUpdate::Failed(reason))
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

    pub fn close_deployment(&mut self, deployment_id: DeploymentId) -> Result<(), ApiError> {
        self.update_deployment_state(deployment_id, DeploymentUpdate::Closed)
    }

    fn update_deployment_state(
        &mut self,
        deployment_id: DeploymentId,
        deployment_update: DeploymentUpdate,
    ) -> Result<(), ApiError> {
        let mut deployment = self.deployments_memory.get(&deployment_id).ok_or_else(|| {
            ApiError::not_found(&format!("Deployment {} not found", deployment_id))
        })?;

        if let DeploymentUpdate::Failed(_) = deployment.state() {
            return Err(ApiError::internal(&format!(
                "Deployment {} already failed",
                deployment_id
            )));
        }

        if let DeploymentUpdate::Closed = deployment.state() {
            return Err(ApiError::internal(&format!(
                "Deployment {} already closed",
                deployment_id
            )));
        }

        deployment.update_state(deployment_update);
        self.deployments_memory.insert(deployment_id, deployment);

        Ok(())
    }
}
