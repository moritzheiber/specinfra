use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

use backend::Backend;
use platform::platform::Platform;
use platform::error::Error;
use provider::Providers;
use provider::file;
use provider::file::FileProvider;
use provider::service;
use provider::service::ServiceProvider;

#[derive(Clone, Debug)]
pub struct RedHat {
    name: String,
    release: String,
}

impl Platform for RedHat {
    fn new() -> RedHat {
        RedHat {
            name: "".to_string(),
            release: "".to_string(),
        }
    }

    fn inline_detector(&self) -> Option<Box<Platform>> {
        let mut file = match File::open("/etc/redhat-release") {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        self.detect_by_redhat_release(&contents)
    }

    fn shell_detector(&self, b: &Backend) -> Option<Box<Platform>> {
        let contents = match b.run_command("cat /etc/redhat-release".into()) {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_redhat_release(&contents.stdout)
    }

    fn get_providers(&self) -> Result<Box<Providers>, Error> {
        let fp = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::linux::Linux),
        };

        let sp = ServiceProvider {
            inline: Box::new(service::inline::systemd::Systemd),
            shell: Box::new(service::shell::systemd::Systemd),
        };

        let p = Providers {
            file: Box::new(fp),
            service: Box::new(sp),
        };

        Ok(Box::new(p))
    }
}

impl RedHat {
    fn detect_by_redhat_release(&self, contents: &str) -> Option<Box<Platform>> {
        let mut line = contents.split(" ");
        let r = RedHat {
            name: line.nth(0).unwrap().trim().to_string(),
            release: line.nth(2).unwrap().trim().to_string(),
        };

        Some(Box::new(r))
    }
}
