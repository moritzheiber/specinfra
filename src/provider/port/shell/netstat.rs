use std::result::Result;

use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::port::shell::ShellProvider;
use crate::provider::Output;

#[derive(Clone, Debug)]
pub struct Netstat;

impl ShellProvider for Netstat {
    fn is_listening(&self, number: usize, b: &dyn Backend) -> Result<Output, Error> {
        let mut c = Command::new("netstat -tunl");
        c.pipe(&format!("grep -- :{}", number));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }
}
