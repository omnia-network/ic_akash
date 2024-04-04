use crate::{
    api::{AkashConfig, ApiError, Config, ConfigService, User, UserId, UserRole, UsersService},
    helpers::EcdsaKeyIds,
};
use candid::Principal;
use ic_cdk::*;

use super::websocket::init_ic_websocket;

#[init]
fn init(is_mainnet: bool) {
    let calling_principal = caller();
    let mut init = Init::default();

    let config = if is_mainnet {
        Config::new_mainnet(
            EcdsaKeyIds::TestKey1,
            "https://rpc.akashnet.net",
            AkashConfig {
                // fetched from https://api.akashnet.net/cosmos/params/v1beta1/params?subspace=deployment&key=MinDeposits
                min_deposit_amount: 500_000,
            },
        )
    } else {
        Config::default()
    };

    if let Err(err) = init.init_config(config) {
        trap(&format!("Error initializing config: {:?}", err));
    }

    if let Err(err) = init.init_admin(calling_principal) {
        trap(&format!("Error initializing admin: {:?}", err));
    }

    init_ic_websocket();
}

#[post_upgrade]
fn post_upgrade() {
    init_ic_websocket();
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
    pub fn init_config(&mut self, config: Config) -> Result<(), ApiError> {
        self.config_service.set_config(config)
    }

    pub fn init_admin(&mut self, principal: Principal) -> Result<UserId, ApiError> {
        let user = User::new(UserRole::Admin);

        self.users_service.create_user(principal, user)
    }
}
