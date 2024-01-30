use std::cell::RefCell;

use candid::Principal;

thread_local! {
    /* flexible */ static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

#[derive(Clone)]
pub struct Config {
    tendermint_rpc_canister_id: Principal,
}

impl Config {
    pub fn new(tendermint_rpc_canister_id: Principal) -> Self {
        Self {
            tendermint_rpc_canister_id,
        }
    }

    pub fn tendermint_rpc_canister_id(&self) -> Principal {
        self.tendermint_rpc_canister_id
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tendermint_rpc_canister_id: Principal::anonymous(),
        }
    }
}

pub fn set_config(config: Config) {
    CONFIG.with_borrow_mut(|c| *c = config)
}

pub fn get_config() -> Config {
    CONFIG.with_borrow(|c| (*c).clone())
}
