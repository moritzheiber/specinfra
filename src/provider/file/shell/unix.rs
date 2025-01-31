use std::result::Result;

use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::error::StringError;
use crate::provider::file::shell::ShellProvider;
use crate::provider::Output;

#[derive(Clone, Debug)]
pub struct Unix;

impl ShellProvider for Unix {
    fn exist(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -e {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn is_file(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -f {}", name));
        self.is_something(name, b, c)
    }

    fn is_directory(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -d {}", name));
        self.is_something(name, b, c)
    }

    fn is_block_device(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -b {}", name));
        self.is_something(name, b, c)
    }

    fn is_character_device(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -c {}", name));
        self.is_something(name, b, c)
    }

    fn is_pipe(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -p {}", name));
        self.is_something(name, b, c)
    }

    fn is_socket(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -S {}", name));
        self.is_something(name, b, c)
    }

    fn is_symlink(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("test -L {}", name));
        self.is_something(name, b, c)
    }

    fn contents(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("cat {}", name));
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn linked_to(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("readlink {}", name));
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }
}

impl Unix {
    pub fn is_something(&self, name: &str, b: &dyn Backend, c: Command) -> Result<Output, Error> {
        let exist = self.exist(name, b)?;
        if Output::to_bool(exist)? {
            let e = StringError {
                string: format!("{} does not exist", name),
            };
            return Err(e.into());
        }

        let res = b.run_command(c);
        let success = match res {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }
}
