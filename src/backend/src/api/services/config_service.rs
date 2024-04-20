use crate::api::{config_state_mut, Config};

#[derive(Default)]
pub struct ConfigService {}

impl ConfigService {
    pub fn set_config(&mut self, config: Config) {
        config_state_mut(|state| *state = config)
    }
}
