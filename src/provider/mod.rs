use std::error::Error;
use std::fmt;
use std::result::Result;

use crate::backend::Backend;

pub struct Providers {
    pub file: Box<file::FileProvider>,
    pub package: Box<package::PackageProvider>,
    pub service: Box<service::ServiceProvider>,
    pub port: Box<port::PortProvider>,
}

pub struct HandleFunc {
    pub inline: Box<dyn Fn() -> Result<Output, error::Error>>,
    pub shell: Box<dyn Fn(&dyn Backend) -> Result<Output, error::Error>>,
}

pub enum Output {
    U32(u32),
    I32(i32),
    I64(i64),
    Bool(bool),
    Text(String),
}

#[derive(Debug)]
pub struct OutputError;

impl Error for OutputError {
    fn description(&self) -> &str {
        "Output error"
    }
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Output error")
    }
}

impl Output {
    pub fn to_u32(o: Output) -> Result<u32, error::Error> {
        match o {
            Output::U32(u) => Ok(u),
            _ => Err(OutputError.into()),
        }
    }

    pub fn to_i32(o: Output) -> Result<i32, error::Error> {
        match o {
            Output::I32(u) => Ok(u),
            _ => Err(OutputError.into()),
        }
    }

    pub fn to_i64(o: Output) -> Result<i64, error::Error> {
        match o {
            Output::I64(i) => Ok(i),
            _ => Err(OutputError.into()),
        }
    }

    pub fn to_bool(o: Output) -> Result<bool, error::Error> {
        match o {
            Output::Bool(b) => Ok(b),
            _ => Err(OutputError.into()),
        }
    }

    pub fn to_string(o: Output) -> Result<String, error::Error> {
        match o {
            Output::Text(s) => Ok(s.to_string()),
            _ => Err(OutputError.into()),
        }
    }
}

pub mod error;
pub mod file;
pub mod package;
pub mod port;
pub mod service;
