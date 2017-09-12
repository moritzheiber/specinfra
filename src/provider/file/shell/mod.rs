use provider::error::Error;
use provider::error::HandleFuncNotDefined;
use provider::Output;
use backend::Backend;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait ShellProvider {
    fn mode(&self, &str, &Backend) -> Result<Output, Error> {
        Err(From::from(HandleFuncNotDefined))
    }

    fn box_clone(&self) -> Box<ShellProvider>;
}

impl Clone for Box<ShellProvider> {
    fn clone(&self) -> Box<ShellProvider> {
        self.box_clone()
    }
}

pub mod bsd;
pub mod null;
