use std::error;
use std::fmt;

#[cfg(all(feature="systemd", target_os="linux"))]
use provider::service::inline::systemd;

#[cfg(not(all(feature="systemd", target_os="linux")))]
use provider::service::inline::_systemd as systemd;

use provider;

#[derive(Debug)]
pub enum Error {
    DBus(systemd::dbus::Error),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DBus(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DBus(ref err) => err.fmt(f),
        }
    }
}

impl From<systemd::dbus::Error> for Error {
    fn from(err: systemd::dbus::Error) -> Error {
        Error::DBus(err)
    }
}

impl From<systemd::dbus::Error> for provider::error::Error {
    fn from(err: systemd::dbus::Error) -> provider::error::Error {
        From::from(Error::DBus(err))
    }
}
