pub mod static_config;
pub mod json_config;

use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ImageConf {
    pub path: String,
    pub dst: (i32, i32),
    pub size: Option<(u32, u32)>,
}

#[derive(Clone, Deserialize)]
pub struct AnalogConf {
    pub image: ImageConf,
    pub range: (i32, i32),
}

#[derive(Clone, Deserialize)]
pub struct Configuration {
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

pub trait ConfigReader {
    fn read_config(&mut self) -> Configuration;
    fn get_path_base(&mut self) -> PathBuf;
}