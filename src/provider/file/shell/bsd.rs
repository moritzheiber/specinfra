use std::result::Result;

use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::file::shell::unix::Unix;
use crate::provider::file::shell::ShellProvider;
use crate::provider::file::Whom;
use crate::provider::Output;

#[derive(Clone, Debug)]
pub struct Bsd;

impl ShellProvider for Bsd {
    fn mode(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("stat -f%Lp {}", name));
        let res = b.run_command(c)?;
        let m = i32::from_str_radix(&res.stdout, 8)?;
        Ok(Output::I32(m))
    }

    fn exist(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.exist(name, b)
    }

    fn is_file(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_file(name, b)
    }

    fn is_directory(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_directory(name, b)
    }

    fn is_block_device(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_block_device(name, b)
    }

    fn is_character_device(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_character_device(name, b)
    }

    fn is_pipe(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_pipe(name, b)
    }

    fn is_socket(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_socket(name, b)
    }

    fn is_symlink(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.is_symlink(name, b)
    }

    fn contents(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.contents(name, b)
    }

    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }

    fn owner(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("stat -f%Su {}", name));
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn group(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("stat -f%Sg {}", name));
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn is_readable(
        &self,
        name: &str,
        whom: Option<&Whom>,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let mode = self.mode(name, b)?;
        let mode_octal = Output::to_i32(mode)?;
        let res = match whom {
            Some(w) => match *w {
                Whom::Owner => Output::Bool(mode_octal & 0o400 != 0),
                Whom::Group => Output::Bool(mode_octal & 0o040 != 0),
                Whom::Others => Output::Bool(mode_octal & 0o004 != 0),
                Whom::User(ref u) => self.is_readable_by_user(name, u, b)?,
            },
            None => Output::Bool(mode_octal & 0o444 != 0),
        };
        Ok(res)
    }

    fn is_writable(
        &self,
        name: &str,
        whom: Option<&Whom>,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let mode = self.mode(name, b)?;
        let mode_octal = Output::to_i32(mode)?;
        let res = match whom {
            Some(w) => match *w {
                Whom::Owner => Output::Bool(mode_octal & 0o200 != 0),
                Whom::Group => Output::Bool(mode_octal & 0o020 != 0),
                Whom::Others => Output::Bool(mode_octal & 0o002 != 0),
                Whom::User(ref u) => self.is_writable_by_user(name, u, b)?,
            },
            None => Output::Bool(mode_octal & 0o222 != 0),
        };
        Ok(res)
    }

    fn md5sum(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let mut c = Command::new(&format!("md5 {}", name));
        c.pipe("awk '{{print $4}}'");
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn sha256sum(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let mut c = Command::new(&format!("shasum -a 256 {}", name));
        c.pipe("awk '{{print $1}}'");
        let res = b.run_command(c)?;
        Ok(Output::Text(res.stdout))
    }

    fn size(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("stat -f%z {}", name));
        let res = b.run_command(c)?;
        Ok(Output::I64(res.stdout.parse::<i64>()?))
    }

    fn linked_to(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        Unix.linked_to(name, b)
    }
}

impl Bsd {
    fn is_readable_by_user(
        &self,
        name: &str,
        user: &str,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let c = Command::new(&format!("sudo -u {} -s test -r {}", user, name));
        Unix.is_something(name, b, c)
    }

    fn is_writable_by_user(
        &self,
        name: &str,
        user: &str,
        b: &dyn Backend,
    ) -> Result<Output, Error> {
        let c = Command::new(&format!("sudo -u {} -s test -w {}", user, name));
        Unix.is_something(name, b, c)
    }
}
