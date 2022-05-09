use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::package::shell::ShellProvider;
use crate::provider::Output;

use std::result::Result;

#[derive(Clone, Debug)]
pub struct Apt;

impl ShellProvider for Apt {
    fn is_installed(
        &self,
        name: &str,
        version: Option<&str>,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let c = match version {
            Some(v) => {
                let mut c = Command::new(&format!(
                    "dpkg-query -f '${{Status}} ${{Version}}' -W {}",
                    name
                ));
                c.pipe(&format!("grep -E '^(install|hold) ok installed {}$'", v));
                c
            }
            None => {
                let mut c = Command::new(&format!("dpkg-query -f '${{Status}}' -W {}", name));
                c.pipe("grep -E '^(install|hold) ok installed$'");
                c
            }
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
                let mut c = Command::new(&format!(
                    "dpkg-query -f '${{Status}} ${{Version}}' -W {}",
                    name
                ));
                c.pipe("sed -n 's/^install ok installed //p'");
                let res = b.run_command(c)?;
                res.stdout
            }
        };
        Ok(Output::Text(v))
    }

    fn install(&self, name: &str, version: Option<&str>, b: &dyn Backend) -> Result<Output, Error> {
        let package = match version {
            Some(v) => [name, v].join("="),
            None => name.to_owned(),
        };
        let c = Command::new(&format!(
            "DEBIAN_FRONTEND='noninteractive' apt-get -y -o \
                                       Dpkg::Options::='--force-confdef' -o \
                                       Dpkg::Options::='--force-confold' install {}",
            package
        ));
        let res = b.run_command(c)?;
        Ok(Output::Bool(res.success))
    }

    fn remove(&self, name: &str, _version: Option<&str>, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!(
            "DEBIAN_FRONTEND='noninteractive' apt-get -y remove {}",
            name
        ));
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
