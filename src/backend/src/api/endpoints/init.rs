use candid::Principal;
use ic_cdk::*;

use crate::{
    api::{ApiError, Config, ConfigService, User, UserId, UserRole, UsersService},
    helpers::EcdsaKeyIds,
};

#[init]
fn init(is_mainnet: bool) {
    let calling_principal = caller();
    let mut init = Init::default();

    if let Err(err) = init.init_admin(calling_principal) {
        trap(&format!("Error initializing admin: {:?}", err));
    }

    let config = if is_mainnet {
        Config::new_mainnet(EcdsaKeyIds::TestKey1, "https://rpc.akashnet.net")
    } else {
        Config::default()
    };

    init.init_config(config);
}

struct Init {
    users_service: UsersService,
    config_service: ConfigService,
}

impl Default for Init {
    fn default() -> Self {
        Self {
            users_service: UsersService::default(),
            config_service: ConfigService::default(),
        }
    }
}

impl Init {
    pub fn init_config(&mut self, config: Config) {
        self.config_service.set_config(config);
    }

    pub fn init_admin(&mut self, principal: Principal) -> Result<UserId, ApiError> {
        let user = User::new(UserRole::Admin);

        self.users_service.create_user(principal, user)
    }
}
