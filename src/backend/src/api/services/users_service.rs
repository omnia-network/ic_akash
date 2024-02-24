use crate::api::{init_users, ApiError, User, UserId, UserRole, UsersMemory};
use candid::Principal;

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

    pub fn change_user_role(
        &mut self,
        user_id: &UserId,
        new_role: UserRole,
    ) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        user.set_role(new_role);

        self.users_memory.insert(*user_id, user);

        Ok(())
    }

    pub fn add_payment_to_user_once(
        &mut self,
        user_id: &UserId,
        payment_block_height: u64,
    ) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        if !user.is_double_payment(payment_block_height) {
            user.add_payment(payment_block_height);
            self.users_memory.insert(*user_id, user);
            return Ok(());
        }
        Err(ApiError::invalid_argument(
            "Payment already used for another deployment",
        ))
    }
}
