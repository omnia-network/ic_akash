use candid::Principal;
use ic_cdk::*;

use crate::api::{
    AccessControlService, ApiError, ApiResult, LedgerService, LogService, User, UserId, UserRole,
    UsersService,
};

#[query]
fn get_user() -> ApiResult<User> {
    let calling_principal = caller();

    UsersEndpoints::default()
        .get_user_by_principal(calling_principal)
        .into()
}

#[update]
fn create_user() -> ApiResult<Principal> {
    let calling_principal = caller();

    UsersEndpoints::default()
        .create_user(calling_principal)
        .map(|user_id| user_id.principal())
        .into()
}

#[update]
fn promote_user_to_admin(admin_principal: Principal) -> ApiResult {
    let calling_principal = caller();

    UsersEndpoints::default()
        .make_user_admin(calling_principal, admin_principal)
        .into()
}

#[update]
async fn update_akt_balance(payment_block_height: u64) -> ApiResult<f64> {
    let calling_principal = caller();

    UsersEndpoints::default()
        .update_akt_balance(calling_principal, payment_block_height)
        .await
        .into()
}

#[derive(Default)]
struct UsersEndpoints {
    users_service: UsersService,
    ledger_service: LedgerService,
    access_control_service: AccessControlService,
    log_service: LogService,
}

impl UsersEndpoints {
    fn get_user_by_principal(&self, calling_principal: Principal) -> Result<User, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        self.users_service.get_user(&calling_principal.into())
    }

    fn create_user(&mut self, calling_principal: Principal) -> Result<UserId, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        let user = User::new(UserRole::Deployer);

        self.users_service.create_user(calling_principal, user)
    }

    fn make_user_admin(
        &mut self,
        calling_principal: Principal,
        admin_principal: Principal,
    ) -> Result<(), ApiError> {
        self.access_control_service
            .assert_principal_is_admin(&calling_principal)?;

        let admin_id = UserId::new(admin_principal);

        self.users_service
            .change_user_role(admin_id, UserRole::Admin)
    }

    async fn update_akt_balance(
        &mut self,
        calling_principal: Principal,
        payment_block_height: u64,
    ) -> Result<f64, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        // check if the payment has been sent from the caller to the orchestrator
        let paid_akt = self
            .ledger_service
            .check_payment(calling_principal, payment_block_height)
            .await?;

        // check if the payment has not been used for a previous deployment by the same user
        let user_id = UserId::new(calling_principal);
        self.users_service
            .add_payment_to_user_once(user_id, payment_block_height, paid_akt)?;

        self.log_service.log_info(
            format!(
                "[User {}]: Received payment of {} AKT. Current balance: {} AKT",
                calling_principal,
                paid_akt,
                self.users_service.get_user_akt_balance(&user_id)?
            ),
            None,
        )?;

        Ok(paid_akt)
    }
}
