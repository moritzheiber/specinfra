use std::fmt::Debug;

use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::Output;

pub trait ShellProvider: Debug {
    fn is_listening(&self, _: usize, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_listening".to_string(),
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

pub mod netstat;
pub mod null;
