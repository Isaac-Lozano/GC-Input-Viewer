use std::error;
use std::fmt;
use std::io::Error as IoError;
use std::result;

use serde_yaml::Error as YamlError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    YamlError(YamlError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO Error: {}", err),
            Error::YamlError(ref err) => write!(f, "Yaml Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::YamlError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::YamlError(ref err) => Some(err),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<YamlError> for Error {
    fn from(err: YamlError) -> Error {
        Error::YamlError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;