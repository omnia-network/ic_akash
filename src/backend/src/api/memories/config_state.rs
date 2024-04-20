use std::cell::RefCell;

use crate::api::Config;

thread_local! {
    /* flexible */ static STATE: RefCell<Config> = RefCell::new(Config::default());
}

pub fn config_state<R>(f: impl FnOnce(&Config) -> R) -> R {
    STATE.with_borrow(|s| f(s))
}

pub fn config_state_mut<R>(f: impl FnOnce(&mut Config) -> R) -> R {
    STATE.with_borrow_mut(|s| f(s))
}
