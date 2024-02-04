use candid::Principal;

use crate::api::{init_users, ApiError, UserId, UsersMemory};

pub struct AccessControlService {
    users_memory: UsersMemory,
}

impl Default for AccessControlService {
    fn default() -> Self {
        Self {
            users_memory: init_users(),
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
}
