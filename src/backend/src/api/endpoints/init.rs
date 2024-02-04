use candid::Principal;
use ic_cdk::*;

use crate::api::{ApiError, User, UserId, UserRole, UsersService};

// TODO: enable this init and remove the root one
// #[init]
// fn init() {
//     let calling_principal = caller();

//     if let Err(err) = Init::default().init_admin(calling_principal) {
//         trap(&format!("Error initializing admin: {:?}", err));
//     }
// }

struct Init {
    users_service: UsersService,
}

impl Default for Init {
    fn default() -> Self {
        Self::new(UsersService::default())
    }
}

impl Init {
    pub fn new(users_service: UsersService) -> Self {
        Self { users_service }
    }
}

impl Init {
    pub fn init_admin(&mut self, principal: Principal) -> Result<UserId, ApiError> {
        let user = User::new(UserRole::Admin);

        self.users_service.create_user(principal, user)
    }
}
