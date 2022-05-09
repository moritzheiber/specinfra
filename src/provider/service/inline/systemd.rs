use crate::provider::error::Error;
use crate::provider::service::inline::InlineProvider;
use crate::provider::Output;

use dbus::arg::Variant;
use dbus::ffidisp::{BusType, Connection, ConnectionItem};
use dbus::{Message, Path};

use std::result::Result;

#[derive(Clone, Debug)]
pub struct Systemd;

impl InlineProvider for Systemd {
    fn is_running(&self, name: &str) -> Result<Output, Error> {
        let state = self.get_active_state(name)?;
        Ok(Output::Bool(state == "active"))
    }

    fn is_enabled(&self, name: &str) -> Result<Output, Error> {
        let state = self.get_unit_file_state(name)?;
        Ok(Output::Bool(state == "enabled"))
    }

    fn enable(&self, name: &str) -> Result<Output, Error> {
        let s = self.enable_unit_file_state(name)?;
        Ok(Output::Bool(s))
    }

    fn disable(&self, name: &str) -> Result<Output, Error> {
        let s = self.disable_unit_file_state(name)?;
        Ok(Output::Bool(s))
    }

    fn start(&self, name: &str) -> Result<Output, Error> {
        let s = self.start_unit(name)?;
        Ok(Output::Bool(s))
    }

    fn stop(&self, name: &str) -> Result<Output, Error> {
        let s = self.stop_unit(name)?;
        Ok(Output::Bool(s))
    }

    fn reload(&self, name: &str) -> Result<Output, Error> {
        let s = self.reload_unit(name)?;
        Ok(Output::Bool(s))
    }

    fn restart(&self, name: &str) -> Result<Output, Error> {
        let s = self.restart_unit(name)?;
        Ok(Output::Bool(s))
    }

    fn box_clone(&self) -> Box<dyn InlineProvider> {
        Box::new((*self).clone())
    }
}

impl Systemd {
    fn start_unit(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;
        let _ = c.add_match("interface='org.freedesktop.systemd1.Manager'");

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "StartUnit",
        )?
        .append2(service.clone(), "replace");

        let _ = c.send(m);
        self.wait_service_job_finished(c, &service, "active")
    }

    fn reload_unit(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;
        let _ = c.add_match("interface='org.freedesktop.systemd1.Manager'");

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "ReloadUnit",
        )?
        .append2(service.clone(), "replace");

        let _ = c.send(m);
        self.wait_service_job_finished(c, &service, "active")
    }

    fn restart_unit(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;
        let _ = c.add_match("interface='org.freedesktop.systemd1.Manager'");

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "RestartUnit",
        )?
        .append2(service.clone(), "replace");

        let _ = c.send(m);
        self.wait_service_job_finished(c, &service, "active")
    }

    fn wait_service_job_finished(
        &self,
        c: Connection,
        service: &str,
        state: &str,
    ) -> Result<bool, Error> {
        for ci in c.iter(10) {
            let m = if let &ConnectionItem::Signal(ref s) = &ci {
                s
            } else {
                continue;
            };

            if &*m.interface().unwrap() != "org.freedesktop.systemd1.Manager" {
                continue;
            };

            if &*m.member().unwrap() != "JobRemoved" {
                continue;
            };

            let (_, _, unit, result) = m.get4::<u32, Path, &str, &str>();

            if unit.unwrap() != service {
                continue;
            };

            if result.unwrap() != "done" {
                continue;
            };

            break;
        }

        if self.get_active_state(service)? == state {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn stop_unit(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;
        let _ = c.add_match("interface='org.freedesktop.systemd1.Manager'");

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "StopUnit",
        )?
        .append2(service.clone(), "replace");

        let _ = c.send(m);
        self.wait_service_job_finished(c, &service, "inactive")
    }

    fn enable_unit_file_state(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "EnableUnitFiles",
        )?
        .append3(vec![service], false, false);

        c.send_with_reply_and_block(m, 2000)?;

        Ok(true)
    }

    fn disable_unit_file_state(&self, name: &str) -> Result<bool, Error> {
        let c = Connection::get_private(BusType::System)?;

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "DisableUnitFiles",
        )?
        .append2(vec![service], false);

        c.send_with_reply_and_block(m, 2000)?;

        Ok(true)
    }

    fn get_active_state(&self, name: &str) -> Result<String, Error> {
        let c = Connection::get_private(BusType::System)?;

        let object_path = self.get_object_path(name)?;

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            object_path,
            "org.freedesktop.DBus.Properties",
            "Get",
        )?
        .append2("org.freedesktop.systemd1.Unit", "ActiveState");

        let r = c.send_with_reply_and_block(m, 2000)?;
        let s: Variant<&str> = r.read1()?;
        let active_state = s.0;

        Ok(active_state.to_string())
    }

    fn get_unit_file_state(&self, name: &str) -> Result<String, Error> {
        let c = Connection::get_private(BusType::System)?;

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "GetUnitFileState",
        )?
        .append1(service);

        let r = c.send_with_reply_and_block(m, 2000)?;
        let unit_file_state: &str = r.read1()?;
        Ok(unit_file_state.to_string())
    }

    fn get_object_path(&self, name: &str) -> Result<Path, Error> {
        let c = Connection::get_private(BusType::System)?;

        let service: String;
        if name.ends_with(".service") {
            service = name.to_string()
        } else {
            service = name.to_string() + ".service"
        }

        let m = Message::new_method_call(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            "org.freedesktop.systemd1.Manager",
            "GetUnit",
        )?
        .append1(service);

        let r = c.send_with_reply_and_block(m, 2000)?;
        let object_path: Path = r.read1()?;
        Ok(object_path)
    }
}
