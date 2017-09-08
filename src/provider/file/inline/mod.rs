use std::error::Error;
use provider::Output;

// See https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
pub enum Whom {
    Owner,
    Group,
    Others,
    User(String),
}

pub trait InlineProvider {
    fn is_file(&self, &str) -> Result<Output, Box<Error>>;
    fn exist(&self, &str) -> Result<Output, Box<Error>>;
    fn is_directory(&self, &str) -> Result<Output, Box<Error>>;
    fn is_block_device(&self, &str) -> Result<Output, Box<Error>>;
    fn is_character_device(&self, &str) -> Result<Output, Box<Error>>;
    fn is_pipe(&self, &str) -> Result<Output, Box<Error>>;
    fn is_socket(&self, &str) -> Result<Output, Box<Error>>;
    fn is_symlink(&self, &str) -> Result<Output, Box<Error>>;
    fn contents(&self, &str) -> Result<Output, Box<Error>>;
    fn mode(&self, &str) -> Result<Output, Box<Error>>;
    fn owner(&self, &str) -> Result<Output, Box<Error>>;
    fn group(&self, &str) -> Result<Output, Box<Error>>;
    fn linked_to(&self, &str) -> Result<Output, Box<Error>>;
    fn is_readable(&self, &str, Option<&Whom>) -> Result<Output, Box<Error>>;
    fn is_writable(&self, &str, Option<&Whom>) -> Result<Output, Box<Error>>;
    fn md5sum(&self, &str) -> Result<Output, Box<Error>>;
    fn sha256sum(&self, &str) -> Result<Output, Box<Error>>;
    fn selinux_label(&self, &str) -> Result<Output, Box<Error>>;
    fn size(&self, &str) -> Result<Output, Box<Error>>;

    fn box_clone(&self) -> Box<InlineProvider>;
}

impl Clone for Box<InlineProvider> {
    fn clone(&self) -> Box<InlineProvider> {
        self.box_clone()
    }
}

pub mod posix;