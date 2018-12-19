use std::error;
use std::fmt;
use std::io::Error as IoError;
use std::result;

use dtm2txt::error::Dtm2txtError as DtmError;
use serde_yaml::Error as YamlError;
use serialport::Error as SerialError;

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    YamlError(YamlError),
    Sdl2Error(Box<dyn error::Error>),
    DtmError(DtmError),
    SerialError(SerialError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO Error: {}", err),
            Error::YamlError(ref err) => write!(f, "Yaml Error: {}", err),
            Error::Sdl2Error(ref err) => write!(f, "Sdl2 Error: {}", err),
            Error::DtmError(ref err) => write!(f, "Dtm Error: {}", err),
            Error::SerialError(ref err) => write!(f, "Serial Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref err) => err.description(),
            Error::YamlError(ref err) => err.description(),
            Error::Sdl2Error(ref err) => err.description(),
            Error::DtmError(ref err) => err.description(),
            Error::SerialError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::YamlError(ref err) => Some(err),
            // TODO: Figure out how to toss out err.
            Error::Sdl2Error(ref _err) => None,
            Error::DtmError(ref err) => Some(err),
            Error::SerialError(ref err) => Some(err),
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

impl From<DtmError> for Error {
    fn from(err: DtmError) -> Error {
        Error::DtmError(err)
    }
}

impl From<SerialError> for Error {
    fn from(err: SerialError) -> Error {
        Error::SerialError(err)
    }
}

pub type Result<T> = result::Result<T, Error>;