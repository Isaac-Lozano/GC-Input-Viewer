use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::configuration::{Configuration, ConfigReader};

pub struct JsonConfig<R> {
    base: PathBuf,
    inner: R,
}

impl<R> JsonConfig<R> {
    pub fn from_read<P>(base: P, inner: R) -> JsonConfig<R>
        where P: AsRef<Path>,
    {
        JsonConfig {
            base: base.as_ref().to_owned(),
            inner: inner,
        }
    }
}

impl JsonConfig<File> {
    pub fn from_path<P>(path: P) -> JsonConfig<File>
        where P: AsRef<Path>,
    {
        let path = path.as_ref();
        let base = path.parent().unwrap();
        let file = File::open(path).unwrap();
        Self::from_read(base, file)
    }
}

impl<R> ConfigReader for JsonConfig<R>
    where R: Read,
{
    fn read_config(&mut self) -> Configuration {
        serde_json::from_reader(&mut self.inner).unwrap()
    }

    fn get_path_base(&mut self) -> PathBuf {
        self.base.clone()
    }
}