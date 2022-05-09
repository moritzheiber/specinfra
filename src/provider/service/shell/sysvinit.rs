use crate::backend::command::Command;
use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::service::shell::ShellProvider;
use crate::provider::Output;

use std::result::Result;

#[derive(Clone, Debug)]
pub struct SysVInit;

impl ShellProvider for SysVInit {
    fn is_running(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} status", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn is_enabled(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let mut c = Command::new(&format!("chkconfig --list {}", name));
        c.pipe("grep 3:on");

        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn disable(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("chkconfig {} off", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn enable(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("chkconfig {} on", name));
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

    fn reload(&self, name: &str, b: &dyn Backend) -> Result<Output, Error> {
        let c = Command::new(&format!("service {} reload", name));
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

    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }
}
