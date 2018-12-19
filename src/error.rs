use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use serde_yaml::Error as YamlError;

#[derive(Debug)]
pub enum ConfigError {
    IoError(IoError),
    YamlError(YamlError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::IoError(ref err) => write!(f, "IO Error: {}", err),
            ConfigError::YamlError(ref err) => write!(f, "Yaml Error: {}", err),
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::IoError(ref err) => err.description(),
            ConfigError::YamlError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::IoError(ref err) => Some(err),
            ConfigError::YamlError(ref err) => Some(err),
        }
    }
}

impl From<IoError> for ConfigError {
    fn from(err: IoError) -> ConfigError {
        ConfigError::IoError(err)
    }
}

impl From<YamlError> for ConfigError {
    fn from(err: YamlError) -> ConfigError {
        ConfigError::YamlError(err)
    }
}

pub type ConfigResult<T> = Result<T, ConfigError>;