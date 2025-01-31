use std::error;
use std::fmt;
use std::io;
use std::num;

use nix;

use crate::backend;
use crate::provider::service;
use crate::provider::OutputError;

#[derive(Debug)]
pub enum Error {
    HandleFuncNotDefined(HandleFuncNotDefined),
    Nix(nix::Error),
    Io(io::Error),
    String(StringError),
    ParseInt(num::ParseIntError),
    Output(OutputError),
    Backend(backend::error::Error),
    Service(service::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HandleFuncNotDefined(ref err) => err.fmt(f),
            Error::Nix(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::String(ref err) => err.fmt(f),
            Error::ParseInt(ref err) => err.fmt(f),
            Error::Output(ref err) => err.fmt(f),
            Error::Backend(ref err) => err.fmt(f),
            Error::Service(ref err) => err.fmt(f),
        }
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Error {
        Error::Nix(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<OutputError> for Error {
    fn from(err: OutputError) -> Error {
        Error::Output(err)
    }
}

impl From<backend::error::Error> for Error {
    fn from(err: backend::error::Error) -> Error {
        Error::Backend(err)
    }
}

impl From<service::error::Error> for Error {
    fn from(err: service::error::Error) -> Error {
        Error::Service(err)
    }
}

#[derive(Debug)]
pub struct HandleFuncNotDefined {
    pub provider: String,
    pub func: String,
}

impl error::Error for HandleFuncNotDefined {
    fn description(&self) -> &str {
        "HandleFunc not found"
    }
}

impl fmt::Display for HandleFuncNotDefined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HandleFunc not found")
    }
}

impl From<HandleFuncNotDefined> for Error {
    fn from(err: HandleFuncNotDefined) -> Error {
        Error::HandleFuncNotDefined(err)
    }
}

#[derive(Debug)]
pub struct StringError {
    pub string: String,
}

impl error::Error for StringError {
    fn description(&self) -> &str {
        &self.string
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl From<StringError> for Error {
    fn from(err: StringError) -> Error {
        Error::String(err)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        let e = StringError { string: s };
        Error::String(e)
    }
}
