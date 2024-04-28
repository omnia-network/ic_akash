use crate::api::{init_users, log_info, ApiError, User, UserId, UserRole, UsersMemory};
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

        self.users_memory.insert(user_id, user);

        Ok(user_id)
    }

    pub fn change_user_role(
        &mut self,
        user_id: UserId,
        new_role: UserRole,
    ) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        user.set_role(new_role);

        self.users_memory.insert(user_id, user);

        Ok(())
    }

    pub fn add_payment_to_user_once(
        &mut self,
        user_id: UserId,
        payment_block_height: u64,
        amount_akt: f64,
    ) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        if user.is_double_payment(payment_block_height) {
            return Err(ApiError::invalid_argument(
                "Payment already used for another deployment",
            ));
        }

        user.add_payment(payment_block_height);
        user.add_to_akt_balance(amount_akt);
        self.users_memory.insert(user_id, user);

        Ok(())
    }

    pub fn get_user_akt_balance(&self, user_id: &UserId) -> Result<f64, ApiError> {
        self.users_memory
            .get(user_id)
            .map(|user| user.akt_balance())
            .ok_or_else(|| ApiError::not_found("User not found"))
    }

    pub fn charge_user(&mut self, user_id: UserId, amount_akt: f64) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        let user_balance = user.akt_balance();
        if user_balance < amount_akt {
            return Err(ApiError::permission_denied(&format!(
                "Not enough AKT balance. Current balance: {} AKT, required: {} AKT",
                user_balance, amount_akt,
            )));
        }

        let updated_balance = user.subtract_from_akt_balance(amount_akt);

        log_info!(
            format!(
                "[User {}]: Updated balance after deployment: {} AKT",
                user_id, updated_balance
            ),
            "charge_user"
        );

        self.users_memory.insert(user_id, user);

        Ok(())
    }
}
