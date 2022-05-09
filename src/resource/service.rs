use libc::c_char;
use std;
use std::ffi::CString;

use crate::backend::Backend;
use crate::provider::error;
use crate::provider::service::ServiceProvider;
use crate::provider::Output;

pub struct Service<'a> {
    name: &'static str,
    backend: &'a dyn Backend,
    provider: &'a ServiceProvider,
    error: Option<error::Error>,
}

impl<'a> Service<'a> {
    pub fn new(n: &'static str, b: &'a dyn Backend, p: &'a ServiceProvider) -> Service<'a> {
        Service {
            name: n,
            backend: b,
            provider: p,
            error: None,
        }
    }

    pub fn is_running(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_running(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_enabled(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_enabled(self.name))
            .and_then(Output::to_bool)
    }

    pub fn enable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.enable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn disable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.disable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn start(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.start(self.name))
            .and_then(Output::to_bool)
    }

    pub fn stop(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.stop(self.name))
            .and_then(Output::to_bool)
    }

    pub fn reload(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.reload(self.name))
            .and_then(Output::to_bool)
    }

    pub fn restart(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.restart(self.name))
            .and_then(Output::to_bool)
    }
}

// Wrapper functions for FFI

#[no_mangle]
pub extern "C" fn resource_service_free(ptr: *mut Service) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn resource_service_error_description(ptr: *const Service) -> *const c_char {
    let s = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    match s.error {
        Some(ref e) => CString::new(e.to_string()).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn resource_service_is_running(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.is_running() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_is_enabled(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.is_enabled() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_enable(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.enable() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_disable(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.disable() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_reload(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.reload() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_restart(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.restart() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_start(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.start() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_service_stop(ptr: *mut Service) -> i32 {
    let s = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match s.stop() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            s.error = Some(e);
            -1
        }
    }
}
