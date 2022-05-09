use crate::backend;
use crate::backend::command;
use crate::backend::command::CommandResult;
use crate::backend::Backend;
use crate::platform::platform::Platform;
use crate::platform::platforms::Platforms;
use crate::provider::error::Error;
use crate::provider::HandleFunc;
use crate::provider::Output;

use std::process::Command;
use std::result::Result;

pub struct Direct;

impl Direct {
    pub fn new() -> Direct {
        Direct
    }
}

impl Backend for Direct {
    fn detect_platform(&self) -> Option<Box<dyn Platform>> {
        let platforms = Platforms::new();
        for p in platforms {
            match p.inline_detector() {
                Some(m) => return Some(m),
                None => (),
            };

            match p.shell_detector(self) {
                Some(m) => return Some(m),
                None => (),
            }
        }
        None
    }

    fn handle(&self, handle_func: Box<HandleFunc>) -> Result<Output, Error> {
        match (handle_func.inline)() {
            Ok(r) => return Ok(r),
            Err(Error::HandleFuncNotDefined(_)) => (),
            Err(e) => return Err(e),
        };

        (handle_func.shell)(self)
    }

    fn run_command(&self, c: command::Command) -> Result<CommandResult, backend::error::Error> {
        let out = Command::new("sh").args(&["-c", &c.string]).output()?;

        if !out.status.success() {
            let e = backend::error::CommandError {
                code: out.status.code().unwrap(),
                message: String::from_utf8(out.stderr)?,
            };
            return Err(e.into());
        }

        let stdout = String::from_utf8(out.stdout)?;
        let stderr = String::from_utf8(out.stderr)?;
        let res = CommandResult {
            stdout: stdout.trim().to_string(),
            stderr: stderr.trim().to_string(),
            code: out.status.code().unwrap(),
            success: out.status.success(),
        };

        Ok(res)
    }
}

// Wrapper functions for FFI

use backend::BackendWrapper;

#[no_mangle]
pub extern "C" fn backend_direct_new() -> *mut BackendWrapper {
    let b = BackendWrapper(Box::new(Direct::new()));
    Box::into_raw(Box::new(b))
}

#[no_mangle]
pub extern "C" fn backend_direct_free(ptr: *mut BackendWrapper) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}
