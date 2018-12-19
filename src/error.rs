use std::error;
use std::fmt;
use std::io::Error as IoError;
use std::result;

use serde_yaml::Error as YamlError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    YamlError(YamlError),
    Sdl2Error(Box<dyn error::Error>)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO Error: {}", err),
            Error::YamlError(ref err) => write!(f, "Yaml Error: {}", err),
            Error::Sdl2Error(ref err) => write!(f, "Sdl2 Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::YamlError(ref err) => err.description(),
            Error::Sdl2Error(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::YamlError(ref err) => Some(err),
            // TODO: Figure out how to toss out err.
            Error::Sdl2Error(ref _err) => None,
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

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Sdl2Error(err.into())
    }
}

pub type Result<T> = result::Result<T, Error>;