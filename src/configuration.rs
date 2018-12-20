use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;

use crate::error::Result;

#[derive(Clone, Debug, Deserialize)]
pub struct ImageConf {
    pub path: String,
    pub dst: (i32, i32),
    pub size: Option<(u32, u32)>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AnalogConf {
    pub image: ImageConf,
    pub range: (i32, i32),
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThemeConfiguration {
    pub size: (u32, u32),
    pub background: ImageConf,
    pub a: Option<ImageConf>,
    pub b: Option<ImageConf>,
    pub x: Option<ImageConf>,
    pub y: Option<ImageConf>,
    pub up: Option<ImageConf>,
    pub down: Option<ImageConf>,
    pub left: Option<ImageConf>,
    pub right: Option<ImageConf>,
    pub start: Option<ImageConf>,
    pub analog: Option<AnalogConf>,
    pub c: Option<AnalogConf>,
    pub l_analog: Option<ImageConf>,
    pub r_analog: Option<ImageConf>,
    pub l_digital: Option<ImageConf>,
    pub r_digital: Option<ImageConf>,
    pub z: Option<ImageConf>,
}

impl ThemeConfiguration {
    pub fn from_read<R>(reader: R) -> Result<ThemeConfiguration>
        where R: Read,
    {
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_path<P>(path: P) -> Result<ThemeConfiguration>
        where P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Self::from_read(file)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum InputSource {
    Dtm(String),
    Sa2(Option<String>),
    Serial(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub theme: ThemeConfiguration,
    pub theme_path: PathBuf,
    pub input: InputSource,
}

impl Configuration {
    pub fn from_read<R>(reader: R) -> Result<Configuration>
        where R: Read,
    {
        let conf_file: ConfigurationFile = serde_yaml::from_reader(reader)?;
        let theme = ThemeConfiguration::from_path(&conf_file.theme_path)?;
        let theme_path = conf_file.theme_path
            .parent()
            .unwrap_or(Path::new("/"))
            .to_owned();

        Ok(Configuration {
            theme: theme,
            theme_path: theme_path,
            input: conf_file.input,
        })
    }

    pub fn from_path<P>(path: P) -> Result<Configuration>
        where P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Self::from_read(file)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigurationFile {
    theme_path: PathBuf,
    input: InputSource,
}
