use std::cell::RefCell;

use crate::ecdsa::EcdsaKeyIds;

thread_local! {
    /* flexible */ static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

#[derive(Clone)]
pub struct Config {
    ecdsa_key: EcdsaKeyIds,
}

impl Config {
    pub fn new(ecdsa_key: EcdsaKeyIds) -> Self {
        Self { ecdsa_key }
    }

    pub fn ecdsa_key(&self) -> EcdsaKeyIds {
        self.ecdsa_key.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ecdsa_key: EcdsaKeyIds::TestKeyLocalDevelopment,
        }
    }
}

pub fn set_config(config: Config) {
    CONFIG.with_borrow_mut(|c| *c = config)
}

pub fn get_config() -> Config {
    CONFIG.with_borrow(|c| (*c).clone())
}
