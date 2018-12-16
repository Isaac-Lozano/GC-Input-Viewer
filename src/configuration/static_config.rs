use crate::configuration::{Configuration, ConfigReader};

pub struct StaticConfig {
    pub conf: Configuration,
}

impl ConfigReader for StaticConfig {
    fn read_config(&mut self) -> Configuration {
        self.conf.clone()
    }
}