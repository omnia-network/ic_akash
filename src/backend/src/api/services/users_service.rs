use crate::api::{init_users, ApiError, User, UserId, UserRole, UsersMemory};
use candid::Principal;
use ic_cdk::print;

const DEPLOYMENT_AKT_PRICE: f64 = 5.0;

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
        akt_amount: f64,
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
        user.add_to_akt_balance(akt_amount);
        self.users_memory.insert(user_id, user);

        Ok(())
    }

    pub fn get_user_akt_balance(&self, user_id: &UserId) -> Result<f64, ApiError> {
        self.users_memory
            .get(user_id)
            .map(|user| user.akt_balance())
            .ok_or_else(|| ApiError::not_found("User not found"))
    }

    pub fn charge_user_for_deployment(&mut self, user_id: UserId) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        let user_balance = user.akt_balance();
        if user_balance < DEPLOYMENT_AKT_PRICE {
            return Err(ApiError::permission_denied(&format!(
                "Not enough AKT balance. Current balance: {} AKT",
                user_balance,
            )));
        }

        let updated_balance = user.subtract_from_akt_balance(DEPLOYMENT_AKT_PRICE);
        print(format!(
            "[{}]: Updated balance after deployment: {} AKT",
            user_id, updated_balance
        ));
        self.users_memory.insert(user_id, user);

        Ok(())
    }

    pub fn charge_user_for_deposit(
        &mut self,
        user_id: UserId,
        amount_akt: f64,
    ) -> Result<(), ApiError> {
        let mut user = self
            .users_memory
            .get(&user_id)
            .ok_or_else(|| ApiError::not_found("User not found"))?;

        let user_balance = user.akt_balance();
        if user_balance < amount_akt {
            return Err(ApiError::permission_denied(&format!(
                "Not enough AKT balance. Current balance: {} AKT",
                user_balance,
            )));
        }

        let updated_balance = user.subtract_from_akt_balance(amount_akt);
        print(format!(
            "[{}]: Updated balance after deposit: {} AKT",
            user_id, updated_balance
        ));
        self.users_memory.insert(user_id, user);

        Ok(())
    }
}
