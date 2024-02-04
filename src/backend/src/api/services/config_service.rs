use crate::api::{init_config, Config, ConfigMemory};

pub struct ConfigService {
    config_memory: ConfigMemory,
}

impl Default for ConfigService {
    fn default() -> Self {
        Self {
            config_memory: init_config(),
        }
    }
}

impl ConfigService {
    pub fn get_config(&self) -> Config {
        self.config_memory.get().clone()
    }

    pub fn set_config(&mut self, config: Config) {
        self.config_memory.set(config);
    }
}
