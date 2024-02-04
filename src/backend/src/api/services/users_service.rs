use candid::Principal;

use crate::api::{init_users, ApiError, User, UserId, UsersMemory};

pub struct UsersService {
    users_memory: UsersMemory,
}

impl Default for UsersService {
    fn default() -> Self {
        Self {
            users_memory: init_users(),
        }
    }
}

impl UsersService {
    pub fn get_user(&self, user_id: &UserId) -> Result<User, ApiError> {
        self.users_memory
            .get(user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))
    }

    pub fn create_user(&mut self, principal: Principal, user: User) -> Result<UserId, ApiError> {
        let user_id = UserId::new(principal);

        self.users_memory.insert(user_id.clone(), user);

        Ok(user_id)
    }

    pub fn make_user_admin(&mut self, user_id: &UserId) -> Result<(), ApiError> {
        self.users_memory
            .get(user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        self.users_memory.insert(user_id.clone(), User::Admin);

        Ok(())
    }
}
