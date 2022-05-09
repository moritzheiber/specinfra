use crate::backend::Backend;
use crate::provider::error::Error;
use crate::provider::error::HandleFuncNotDefined;
use crate::provider::file::Whom;
use crate::provider::Output;

use std::fmt::Debug;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6

pub trait ShellProvider: Debug {
    fn mode(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "mode".to_string(),
        };
        Err(e.into())
    }

    fn size(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "size".to_string(),
        };
        Err(e.into())
    }

    fn is_file(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_file".to_string(),
        };
        Err(e.into())
    }

    fn is_directory(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_directory".to_string(),
        };
        Err(e.into())
    }

    fn is_block_device(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_block_device".to_string(),
        };
        Err(e.into())
    }

    fn is_character_device(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_character_device".to_string(),
        };
        Err(e.into())
    }

    fn is_pipe(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_pipe".to_string(),
        };
        Err(e.into())
    }

    fn is_socket(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_socket".to_string(),
        };
        Err(e.into())
    }

    fn is_symlink(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_symlink".to_string(),
        };
        Err(e.into())
    }

    fn exist(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "exist".to_string(),
        };
        Err(e.into())
    }

    fn contents(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "contents".to_string(),
        };
        Err(e.into())
    }

    fn owner(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "owner".to_string(),
        };
        Err(e.into())
    }

    fn group(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "group".to_string(),
        };
        Err(e.into())
    }

    fn linked_to(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "linked_to".to_string(),
        };
        Err(e.into())
    }

    fn is_readable(&self, _: &str, _: Option<&Whom>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_readable".to_string(),
        };
        Err(e.into())
    }

    fn is_writable(&self, _: &str, _: Option<&Whom>, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "is_writable".to_string(),
        };
        Err(e.into())
    }

    fn md5sum(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "md5sum".to_string(),
        };
        Err(e.into())
    }

    fn sha256sum(&self, _: &str, _: &dyn Backend) -> Result<Output, Error> {
        let e = HandleFuncNotDefined {
            provider: format!("{:?}", self),
            func: "sha256sum".to_string(),
        };
        Err(e.into())
    }

    fn box_clone(&self) -> Box<dyn ShellProvider>;
}

impl Clone for Box<dyn ShellProvider> {
    fn clone(&self) -> Box<dyn ShellProvider> {
        self.box_clone()
    }
}

pub mod bsd;
pub mod linux;
pub mod null;
pub mod unix;
