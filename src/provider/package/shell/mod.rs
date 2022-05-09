use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::Output;

use std::fmt::Debug;

pub trait ShellProvider: Debug {
    fn is_installed(&self, _: &str, _: Option<&str>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_installed".to_string(),
        };
        Err(e.into())
    }

    fn version(&self, _: &str, _: Option<&str>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "version".to_string(),
        };
        Err(e.into())
    }

    fn remove(&self, _: &str, _: Option<&str>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "remove".to_string(),
        };
        Err(e.into())
    }

    fn install(&self, _: &str, _: Option<&str>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "install".to_string(),
        };
        Err(e.into())
    }

    fn box_clone(&self) -> Box<dyn ShellProvider>;
}

impl Clone for Box<dyn ShellProvider> {
    fn clone(&self) -> Box<dyn ShellProvider> {
        self.box_clone()
    }
}

pub mod apt;
pub mod null;
pub mod yum;
