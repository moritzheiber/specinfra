use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::Output;

use std::fmt::Debug;

pub trait InlineProvider: Debug {
    fn is_installed(&self, _: &str, _: Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_installed".to_string(),
        };
        Err(e.into())
    }

    fn version(&self, _: &str, _: Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "version".to_string(),
        };
        Err(e.into())
    }

    fn remove(&self, _: &str, _: Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "remove".to_string(),
        };
        Err(e.into())
    }

    fn install(&self, _: &str, _: Option<&str>) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "install".to_string(),
        };
        Err(e.into())
    }

    fn box_clone(&self) -> Box<dyn InlineProvider>;
}

impl Clone for Box<dyn InlineProvider> {
    fn clone(&self) -> Box<dyn InlineProvider> {
        self.box_clone()
    }
}

pub mod null;
