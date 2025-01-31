use crate::backend::Backend;
use crate::platform::error::Error;
use crate::platform::platform::Platform;
use crate::provider::file;
use crate::provider::file::FileProvider;
use crate::provider::package;
use crate::provider::package::PackageProvider;
use crate::provider::port;
use crate::provider::port::PortProvider;
use crate::provider::service;
use crate::provider::service::ServiceProvider;
use crate::provider::Providers;

use version_compare::Version;

use std::fs::File;
use std::io::prelude::*;
use std::result::Result;

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

    fn inline_detector(&self) -> Option<Box<dyn Platform>> {
        let mut file = match File::open("/etc/redhat-release") {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        self.detect_by_redhat_release(&contents)
    }

    fn shell_detector(&self, b: &dyn Backend) -> Option<Box<dyn Platform>> {
        let contents = match b.run_command("cat /etc/redhat-release".into()) {
            Err(_) => return None,
            Ok(f) => f,
        };

        self.detect_by_redhat_release(&contents.stdout)
    }

    fn get_providers(&self) -> Result<Box<Providers>, Error> {
        let file_provider = FileProvider {
            inline: Box::new(file::inline::posix::Posix),
            shell: Box::new(file::shell::linux::Linux),
        };

        let r = Version::from(&self.release).unwrap();
        let r7 = Version::from("7").unwrap();

        let service_provider = match r {
            ref n if n >= &r7 => ServiceProvider {
                inline: Box::new(service::inline::systemd::Systemd),
                shell: Box::new(service::shell::systemd::Systemd),
            },
            _ => ServiceProvider {
                inline: Box::new(service::inline::null::Null),
                shell: Box::new(service::shell::sysvinit::SysVInit),
            },
        };

        let package_provider = PackageProvider {
            inline: Box::new(package::inline::null::Null),
            shell: Box::new(package::shell::yum::Yum),
        };

        let port_provider = PortProvider {
            inline: Box::new(port::inline::null::Null),
            shell: Box::new(port::shell::netstat::Netstat),
        };

        let p = Providers {
            file: Box::new(file_provider),
            service: Box::new(service_provider),
            package: Box::new(package_provider),
            port: Box::new(port_provider),
        };

        Ok(Box::new(p))
    }
}

impl RedHat {
    fn detect_by_redhat_release(&self, contents: &str) -> Option<Box<dyn Platform>> {
        let mut line = contents.split(' ');
        let name = line.next().unwrap().trim().to_string();
        let mut release = line.nth(1).unwrap().trim().to_string();
        if release == "release" {
            release = line.next().unwrap().trim().to_string();
        }

        let r = RedHat { name, release };

        Some(Box::new(r))
    }
}
