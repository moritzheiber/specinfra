use crate::backend::command::Command;
use crate::backend::command::CommandResult;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::service::shell::ShellProvider;
use crate::provider::Output;

use std::result::Result;

#[derive(Clone, Debug)]
pub struct UbuntuInit;

impl ShellProvider for UbuntuInit {
    fn is_running(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} status", name));
        let res: CommandResult;

        match b.run_command(c) {
            Ok(r) => res = r,
            Err(_) => return Ok(Output::Bool(false)),
        }

        // Ubuntu trusty falls back to upstart and returns exit code 0
        // even though the service is stopped.
        // So we must check the string of stdout.
        if res.stdout.contains("stop") {
            return Ok(Output::Bool(false));
        }

        Ok(Output::Bool(true))
    }

    fn is_enabled(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let mut c = Command::new("ls /etc/rc3.d/");
        c.pipe(&format!("grep -- '^S..{}$'", name));
        c.or(&format!("grep '^\\s*start on' /etc/init/{}.conf", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn disable(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("update-rc.d -f {} remove", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn enable(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("update-rc.d {} defaults", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn start(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} start", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn stop(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} stop", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn restart(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} restart", name));

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };

        Ok(Output::Bool(success))
    }

    fn reload(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} reload", name));

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
