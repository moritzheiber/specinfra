use crate::backend::command::Command;
use crate::backend::command::CommandResult;
use crate::platform::platform::Platform;
use crate::provider;
use crate::provider::HandleFunc;
use crate::provider::Output;

use std::result::Result;

pub trait Backend {
    fn detect_platform(&self) -> Option<Box<dyn Platform>>;
    fn handle(&self, _: Box<HandleFunc>) -> Result<Output, provider::error::Error>;
    fn run_command(&self, _: Command) -> Result<CommandResult, error::Error>;
}

// Wrapper struct for FFI
pub struct BackendWrapper(pub Box<dyn Backend>);

pub mod command;
pub mod direct;
pub mod error;

#[cfg(feature = "backend-ssh")]
pub mod ssh;

#[cfg(not(feature = "backend-ssh"))]
pub mod _ssh;

#[cfg(not(feature = "backend-ssh"))]
pub use self::_ssh as ssh;
