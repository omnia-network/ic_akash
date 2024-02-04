use crate::api::{init_config, ApiError, Config, ConfigMemory};

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

    pub fn set_config(&mut self, config: Config) -> Result<(), ApiError> {
        self.config_memory
            .set(config)
            .map(|_| ())
            .map_err(|e| ApiError::internal(&format!("Error setting config in memory: {:?}", e)))
    }
}
