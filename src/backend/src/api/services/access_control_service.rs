use candid::Principal;

use crate::api::{
    init_deployments, init_users, ApiError, DeploymentId, DeploymentsMemory, UserId, UsersMemory,
};

pub struct AccessControlService {
    users_memory: UsersMemory,
    deployments_memory: DeploymentsMemory,
}

impl Default for AccessControlService {
    fn default() -> Self {
        Self {
            users_memory: init_users(),
            deployments_memory: init_deployments(),
        }
    }
}

impl AccessControlService {
    pub fn assert_principal_not_anonymous(&self, principal: &Principal) -> Result<(), ApiError> {
        if principal == &Principal::anonymous() {
            return Err(ApiError::unauthenticated());
        }

        Ok(())
    }

    pub fn assert_principal_is_admin(&self, principal: &Principal) -> Result<(), ApiError> {
        let user_id = UserId::new(*principal);

        let user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found(format!("User {} not found", user_id).as_str()))?;

        if !user.is_admin() {
            return Err(ApiError::permission_denied(&format!(
                "Principal {} is not an admin",
                principal
            )));
        }

        Ok(())
    }

    pub fn assert_user_owns_deployment(
        &self,
        user_principal: &Principal,
        deployment_id: &DeploymentId,
    ) -> Result<(), ApiError> {
        let user_id = UserId::new(*user_principal);

        let deployment = self.deployments_memory.get(deployment_id).ok_or_else(|| {
            ApiError::not_found(&format!("Deployment {} not found", deployment_id))
        })?;

        if !deployment.user_owns_deployment(&user_id) {
            return Err(ApiError::permission_denied(&format!(
                "User {} does not own deployment {}",
                user_id, deployment_id
            )));
        }

        Ok(())
    }
}
