use crate::api::{config_state_mut, Config};

pub struct ConfigService {}

impl Default for ConfigService {
    fn default() -> Self {
        Self {}
    }
}

impl ConfigService {
    pub fn set_config(&mut self, config: Config) {
        config_state_mut(|state| *state = config)
    }
}
