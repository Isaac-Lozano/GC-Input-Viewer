use std::path::PathBuf;

use crate::configuration::{Configuration, ConfigReader};

pub struct StaticConfig {
    pub conf: Configuration,
    pub base: PathBuf,
}

impl ConfigReader for StaticConfig {
    fn read_config(&mut self) -> Configuration {
        self.conf.clone()
    }

    fn get_path_base(&mut self) -> PathBuf {
        self.base.clone()
    }
}