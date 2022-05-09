use std::fmt::Debug;

use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::Output;

pub trait InlineProvider: Debug {
    fn is_listening(&self, _: usize) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_listening".to_string(),
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
