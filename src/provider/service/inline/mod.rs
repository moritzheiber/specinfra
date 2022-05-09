use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::Output;

use std::fmt::Debug;

pub trait InlineProvider: Debug {
    fn is_running(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_running".to_string(),
        };
        Err(e.into())
    }

    fn is_enabled(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_enabled".to_string(),
        };
        Err(e.into())
    }

    fn enable(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "enable".to_string(),
        };
        Err(e.into())
    }

    fn disable(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "disable".to_string(),
        };
        Err(e.into())
    }

    fn start(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "start".to_string(),
        };
        Err(e.into())
    }

    fn stop(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "stop".to_string(),
        };
        Err(e.into())
    }

    fn reload(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "reload".to_string(),
        };
        Err(e.into())
    }

    fn restart(&self, _: &str) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "restart".to_string(),
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
pub mod systemd;
