extern crate libc;
extern crate md5;
extern crate nix;
extern crate sha2;
extern crate uname;
extern crate users;
extern crate version_compare;

use std::ffi::CStr;

use libc::c_char;

use backend::Backend;
use platform::error::DetectError;
use platform::error::Error;
use platform::platform::Platform;
use provider::Providers;
use resource::file::File;
use resource::package::Package;
use resource::port::Port;
use resource::service::Service;

pub struct Specinfra<'a> {
    pub backend: &'a dyn Backend,
    pub platform: Box<dyn Platform>,
    pub providers: Box<Providers>,
}

pub fn new(b: &dyn Backend) -> Result<Specinfra, Error> {
    let p = b.detect_platform().ok_or(DetectError {
        message: "Failed to detect platform".to_string(),
    })?;
    let providers = p.get_providers()?;
    Ok(Specinfra {
        backend: b,
        platform: p,
        providers,
    })
}

impl<'a> Specinfra<'a> {
    pub fn file(&self, name: &'static str) -> File {
        File::new(name, self.backend, &self.providers.file)
    }

    pub fn service(&self, name: &'static str) -> Service {
        Service::new(name, self.backend, &self.providers.service)
    }

    pub fn package(&self, name: &'static str, version: Option<&'static str>) -> Package {
        Package::new(name, version, self.backend, &self.providers.package)
    }

    pub fn port(&self, number: usize) -> Port {
        Port::new(number, self.backend, &self.providers.port)
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn specinfra_new<'a>(ptr: *const BackendWrapper) -> *const Specinfra<'a> {
    let b = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    Box::into_raw(Box::new(self::new(&*b.0).unwrap()))
}

#[no_mangle]
pub extern "C" fn specinfra_free(ptr: *mut Specinfra) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn specinfra_file(ptr: *const Specinfra, name: *const c_char) -> *const File {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };

    Box::into_raw(Box::new(s.file(name.to_str().unwrap())))
}

#[no_mangle]
pub extern "C" fn specinfra_service(ptr: *const Specinfra, name: *const c_char) -> *const Service {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };

    Box::into_raw(Box::new(s.service(name.to_str().unwrap())))
}

pub mod backend;
pub mod error;
pub mod platform;
pub mod provider;
pub mod resource;
