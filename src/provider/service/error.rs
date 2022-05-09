use crate::provider;

use std::fmt;

#[derive(Debug)]
pub enum Error {
    DBus(dbus::Error),
    DBusArgTypeMismatch(dbus::arg::TypeMismatchError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DBus(ref err) => err.fmt(f),
            Error::DBusArgTypeMismatch(ref err) => err.fmt(f),
        }
    }
}

impl From<dbus::Error> for Error {
    fn from(err: dbus::Error) -> Error {
        Error::DBus(err)
    }
}

impl From<dbus::arg::TypeMismatchError> for Error {
    fn from(err: dbus::arg::TypeMismatchError) -> Error {
        Error::DBusArgTypeMismatch(err)
    }
}

impl From<dbus::Error> for provider::error::Error {
    fn from(err: dbus::Error) -> provider::error::Error {
        Error::DBus(err).into()
    }
}

impl From<dbus::arg::TypeMismatchError> for provider::error::Error {
    fn from(err: dbus::arg::TypeMismatchError) -> provider::error::Error {
        Error::DBusArgTypeMismatch(err).into()
    }
}
