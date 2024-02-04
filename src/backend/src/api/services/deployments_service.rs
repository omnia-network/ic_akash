use crate::api::{init_deployments, ApiError, Deployment, DeploymentId, DeploymentsMemory, UserId};

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

    pub async fn create_deployment(
        &mut self,
        deployment: Deployment,
    ) -> Result<DeploymentId, ApiError> {
        let deployment_id = DeploymentId::new()
            .await
            .map_err(|e| ApiError::internal(&format!("Failed to create deployment id: {}", e)))?;

        self.deployments_memory
            .insert(deployment_id.clone(), deployment);

        Ok(deployment_id)
    }
}