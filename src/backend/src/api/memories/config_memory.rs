use ic_stable_structures::Cell;

use crate::api::Config;

use super::{Memory, CONFIG_MEMORY_ID, MEMORY_MANAGER};

pub type ConfigMemory = Cell<Config, Memory>;

pub fn init_config() -> ConfigMemory {
    ConfigMemory::init(get_config_memory(), Config::default()).unwrap()
}

fn get_config_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CONFIG_MEMORY_ID))
}
