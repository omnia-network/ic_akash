use std::cell::RefCell;

use crate::ecdsa::EcdsaKeyIds;

thread_local! {
    /* flexible */ static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

#[derive(Clone)]
pub struct Config {
    is_mainnet: bool,
    ecdsa_key: EcdsaKeyIds,
    tendermint_rpc_url: String,
}

impl Config {
    pub fn new(is_mainnet: bool, ecdsa_key: EcdsaKeyIds, tendermint_rpc_url: &str) -> Self {
        Self {
            is_mainnet,
            ecdsa_key,
            tendermint_rpc_url: tendermint_rpc_url.to_string(),
        }
    }

    pub fn is_mainnet(&self) -> bool {
        self.is_mainnet
    }

    pub fn ecdsa_key(&self) -> EcdsaKeyIds {
        self.ecdsa_key.clone()
    }

    pub fn tendermint_rpc_url(&self) -> String {
        self.tendermint_rpc_url.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            is_mainnet: false,
            ecdsa_key: EcdsaKeyIds::TestKeyLocalDevelopment,
            tendermint_rpc_url: "https://rpc.sandbox-01.aksh.pw".to_string(),
        }
    }
}

pub fn set_config(config: Config) {
    CONFIG.with_borrow_mut(|c| *c = config)
}

pub fn get_config() -> Config {
    CONFIG.with_borrow(|c| (*c).clone())
}
