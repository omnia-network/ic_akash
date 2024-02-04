use candid::Principal;
use ic_cdk::*;

use crate::api::{AccessControlService, ApiError, ApiResult, User, UserId, UserRole, UsersService};

#[query]
fn get_user() -> ApiResult<User> {
    let calling_principal = caller();

    UsersEndpoints::default()
        .get_user_by_principal(calling_principal)
        .into()
}

#[update]
fn create_user() -> ApiResult<UserId> {
    let calling_principal = caller();

    UsersEndpoints::default()
        .create_user(calling_principal)
        .into()
}

#[update]
fn promote_user_to_admin(admin_principal: Principal) -> ApiResult {
    let calling_principal = caller();

    UsersEndpoints::default()
        .make_user_admin(calling_principal, admin_principal)
        .into()
}

struct UsersEndpoints {
    users_service: UsersService,
    access_control_service: AccessControlService,
}

impl Default for UsersEndpoints {
    fn default() -> Self {
        Self {
            users_service: UsersService::default(),
            access_control_service: AccessControlService::default(),
        }
    }
}

impl UsersEndpoints {
    pub fn get_user_by_principal(&self, calling_principal: Principal) -> Result<User, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        self.users_service.get_user(&calling_principal.into())
    }

    pub fn create_user(&mut self, calling_principal: Principal) -> Result<UserId, ApiError> {
        self.access_control_service
            .assert_principal_not_anonymous(&calling_principal)?;

        let user = User::new(UserRole::Deployer);

        self.users_service.create_user(calling_principal, user)
    }

    pub fn make_user_admin(
        &mut self,
        calling_principal: Principal,
        admin_principal: Principal,
    ) -> Result<(), ApiError> {
        self.access_control_service
            .assert_principal_is_admin(&calling_principal)?;

        let admin_id = UserId::new(admin_principal);

        self.users_service
            .change_user_role(&admin_id, UserRole::Admin)
    }
}
