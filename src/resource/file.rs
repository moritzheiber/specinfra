use libc::c_char;
use std;
use std::ffi::CStr;
use std::ffi::CString;

use crate::backend::Backend;
use crate::provider::error;
use crate::provider::file::FileProvider;
use crate::provider::Output;

pub struct File<'a> {
    name: &'static str,
    backend: &'a dyn Backend,
    provider: &'a FileProvider,
    error: Option<error::Error>,
}

impl<'a> File<'a> {
    pub fn new(n: &'static str, b: &'a dyn Backend, p: &'a FileProvider) -> File<'a> {
        File {
            name: n,
            backend: b,
            provider: p,
            error: None,
        }
    }

    pub fn mode(&self) -> Result<i32, error::Error> {
        self.backend
            .handle(self.provider.mode(self.name))
            .and_then(Output::to_i32)
    }

    pub fn size(&self) -> Result<i64, error::Error> {
        self.backend
            .handle(self.provider.size(self.name))
            .and_then(Output::to_i64)
    }

    pub fn is_file(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_file(self.name))
            .and_then(Output::to_bool)
    }

    pub fn exist(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.exist(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_directory(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_directory(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_block_device(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_block_device(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_character_device(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_character_device(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_pipe(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_pipe(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_socket(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_socket(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_symlink(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_symlink(self.name))
            .and_then(Output::to_bool)
    }

    pub fn contents(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.contents(self.name))
            .and_then(Output::to_string)
    }

    pub fn owner(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.owner(self.name))
            .and_then(Output::to_string)
    }

    pub fn group(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.group(self.name))
            .and_then(Output::to_string)
    }

    pub fn linked_to(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.linked_to(self.name))
            .and_then(Output::to_string)
    }

    pub fn is_readable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_readable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_owner(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_readable_by_owner(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_group(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_readable_by_group(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_others(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_readable_by_others(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_readable_by_user(&self, user: &'static str) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_readable_by_user(self.name, user))
            .and_then(Output::to_bool)
    }

    pub fn is_writable(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_writable(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_owner(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_writable_by_owner(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_group(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_writable_by_group(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_others(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_writable_by_others(self.name))
            .and_then(Output::to_bool)
    }

    pub fn is_writable_by_user(&self, user: &'static str) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_writable_by_user(self.name, user))
            .and_then(Output::to_bool)
    }

    pub fn md5sum(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.md5sum(self.name))
            .and_then(Output::to_string)
    }

    pub fn sha256sum(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.sha256sum(self.name))
            .and_then(Output::to_string)
    }
}

// Wrapper functions for FFI

#[no_mangle]
pub extern "C" fn resource_file_free(ptr: *mut File) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn resource_file_error_description(ptr: *const File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    match f.error {
        Some(ref e) => CString::new(e.to_string()).unwrap().into_raw(),
        None => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn resource_file_mode(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    f.mode().unwrap_or_else(|e| {
        f.error = Some(e);
        -1
    })
}

#[no_mangle]
pub extern "C" fn resource_file_is_file(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_file() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_exist(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.exist() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_directory(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_directory() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_block_device(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_block_device() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_character_device(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_character_device() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_pipe(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_pipe() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_socket(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_socket() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_symlink(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_symlink() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_contents(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.contents() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_owner(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.owner() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_group(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.group() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_readable() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_owner(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_readable_by_owner() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_group(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_readable_by_group() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_others(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_readable_by_others() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_readable_by_user(ptr: *mut File, u: *const c_char) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let c_str = unsafe {
        assert!(!u.is_null());
        CStr::from_ptr(u)
    };

    let user = c_str.to_str().unwrap();
    match f.is_readable_by_user(user) {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_writable() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_owner(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_writable_by_owner() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_group(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_writable_by_group() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_others(ptr: *mut File) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.is_writable_by_others() {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_is_writable_by_user(ptr: *mut File, u: *const c_char) -> i32 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let c_str = unsafe {
        assert!(!u.is_null());
        CStr::from_ptr(u)
    };

    let user = c_str.to_str().unwrap();
    match f.is_writable_by_user(user) {
        Ok(f) => {
            if f {
                1
            } else {
                0
            }
        }
        Err(e) => {
            f.error = Some(e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_md5sum(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.md5sum() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_sha256sum(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.sha256sum() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn resource_file_size(ptr: *mut File) -> i64 {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    f.size().unwrap_or_else(|e| {
        f.error = Some(e);
        -1
    })
}

#[no_mangle]
pub extern "C" fn resource_file_linked_to(ptr: *mut File) -> *const c_char {
    let f = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match f.linked_to() {
        Ok(c) => CString::new(c).unwrap().into_raw(),
        Err(e) => {
            f.error = Some(e);
            std::ptr::null()
        }
    }
}
