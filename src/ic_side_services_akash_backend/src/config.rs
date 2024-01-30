use std::cell::RefCell;

thread_local! {
    /* flexible */ static CONFIG: RefCell<Config> = RefCell::new(Config::default());
}

#[derive(Clone)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub fn set_config(config: Config) {
    CONFIG.with_borrow_mut(|c| *c = config)
}

pub fn get_config() -> Config {
    CONFIG.with_borrow(|c| (*c).clone())
}
