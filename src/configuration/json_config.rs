use std::io::Read;
use std::fs::File;

use crate::configuration::{Configuration, ConfigReader};

pub struct JsonConfig<R> {
    inner: R,
}

impl<R> JsonConfig<R> {
    pub fn from_read(inner: R) -> JsonConfig<R> {
        JsonConfig {
            inner: inner,
        }
    }
}

impl JsonConfig<File> {
    pub fn from_path(path: &str) -> JsonConfig<File> {
        let file = File::open(path).unwrap();
        Self::from_read(file)
    }
}

impl<R> ConfigReader for JsonConfig<R>
    where R: Read,
{
    fn read_config(&mut self) -> Configuration {
        serde_json::from_reader(&mut self.inner).unwrap()
    }
}