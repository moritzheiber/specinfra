use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::package::shell::ShellProvider;
use crate::provider::Output;

use std::result::Result;

#[derive(Clone, Debug)]
pub struct Yum;

impl ShellProvider for Yum {
    fn is_installed(
        &self,
        name: &str,
        version: Option<&str>,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let mut c = Command::new(&format!("rpm -q {}", name));

        match version {
            Some(v) => {
                let full_package = [name, v].join("-");
                c.pipe(&format!("grep -w -- {}", full_package));
            }
            None => (),
        };

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn version(&self, name: &str, version: Option<&str>, b: &dyn Backend) -> Result<Output, Error> {
        let v = match version {
            Some(v) => v.to_owned(),
            None => {
                let c = Command::new(&format!("rpm -q --qf '%{{VERSION}}-%{{RELEASE}}' {}", name));
                let res = b.run_command(c)?;
                res.stdout
            }
        };
        Ok(Output::Text(v))
    }

    fn remove(&self, name: &str, _version: Option<&str>, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("yum -y remove {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn install(&self, name: &str, version: Option<&str>, b: &dyn Backend) -> Result<Output, Error> {
        let package = match version {
            Some(v) => [name, v].join("-"),
            None => name.to_owned(),
        };
        let c = Command::new(&format!("yum -y install {}", package));
        let res = b.run_command(c)?;
        Ok(Output::Bool(res.success))
    }

    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }
}
