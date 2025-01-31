use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path;
use std::result::Result;

use crate::provider::error::Error;
use crate::provider::error::StringError;
use crate::provider::file::inline::{InlineProvider, Whom};
use crate::provider::Output;

use users;
use users::os::unix::GroupExt;

use md5;
use sha2::{self, Digest, Sha256};

use nix;

#[derive(Clone, Debug)]
pub struct Posix;

impl Posix {
    fn file_owner(&self, name: &str) -> Result<users::User, Error> {
        let uid = fs::metadata(name).map(|m| MetadataExt::uid(&m))?;
        let owner = users::get_user_by_uid(uid).ok_or(StringError {
            string: format!("Failed to get user from uid: {}", uid),
        })?;
        Ok(owner)
    }

    fn file_group(&self, name: &str) -> Result<users::Group, Error> {
        let gid = fs::metadata(name).map(|m| MetadataExt::gid(&m))?;
        let group = users::get_group_by_gid(gid).ok_or(StringError {
            string: format!("Failed to get group from gid: {}", gid),
        })?;
        Ok(group)
    }

    fn file_content(&self, name: &str) -> Result<String, Error> {
        let file = fs::File::open(name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(contents)
    }

    // FIXME: is_readableとis_writableをまとめる
    fn is_readable_by_user(&self, name: &str, user: &str) -> Result<Output, Error> {
        let file_owner = self.file_owner(name)?;
        if file_owner.name() == user {
            self.is_readable(name, Some(&Whom::Owner))
        } else {
            let file_group = self.file_group(name)?;
            let file_group_members = GroupExt::members(&file_group);
            if file_group_members.contains(&user.to_owned().into()) {
                self.is_readable(name, Some(&Whom::Group))
            } else {
                self.is_readable(name, Some(&Whom::Others))
            }
        }
    }

    fn is_writable_by_user(&self, name: &str, user: &str) -> Result<Output, Error> {
        let file_owner = self.file_owner(name)?;

        if file_owner.name() == user {
            self.is_writable(name, Some(&Whom::Owner))
        } else {
            let file_group = self.file_group(name)?;
            let file_group_members = GroupExt::members(&file_group);
            if file_group_members.contains(&user.to_owned().into()) {
                self.is_writable(name, Some(&Whom::Group))
            } else {
                self.is_writable(name, Some(&Whom::Others))
            }
        }
    }
}

impl InlineProvider for Posix {
    fn mode(&self, name: &str) -> Result<Output, Error> {
        let res =
            fs::metadata(name).map(|m| Output::I32((m.permissions().mode() & 0o7777) as i32))?;
        Ok(res)
    }

    fn is_file(&self, name: &str) -> Result<Output, Error> {
        let res = fs::metadata(name).map(|m| Output::Bool(m.is_file()))?;
        Ok(res)
    }

    fn exist(&self, name: &str) -> Result<Output, Error> {
        let res = Output::Bool(path::Path::new(name).exists());
        Ok(res)
    }

    fn is_directory(&self, name: &str) -> Result<Output, Error> {
        let res = fs::metadata(name).map(|m| Output::Bool(m.is_dir()))?;
        Ok(res)
    }

    fn is_block_device(&self, name: &str) -> Result<Output, Error> {
        let res = fs::metadata(name)
            .map(|m| Output::Bool(FileTypeExt::is_block_device(&m.file_type())))?;
        Ok(res)
    }

    fn is_character_device(&self, name: &str) -> Result<Output, Error> {
        let res = fs::metadata(name)
            .map(|m| Output::Bool(FileTypeExt::is_char_device(&m.file_type())))?;
        Ok(res)
    }

    fn is_pipe(&self, name: &str) -> Result<Output, Error> {
        let res = fs::metadata(name).map(|m| Output::Bool(FileTypeExt::is_fifo(&m.file_type())))?;
        Ok(res)
    }

    fn is_socket(&self, name: &str) -> Result<Output, Error> {
        let res =
            fs::metadata(name).map(|m| Output::Bool(FileTypeExt::is_socket(&m.file_type())))?;
        Ok(res)
    }

    fn is_symlink(&self, name: &str) -> Result<Output, Error> {
        let res = fs::symlink_metadata(name).map(|m| Output::Bool(m.file_type().is_symlink()))?;
        Ok(res)
    }

    fn contents(&self, name: &str) -> Result<Output, Error> {
        let string_content = self.file_content(name)?;
        let res = Output::Text(string_content);
        Ok(res)
    }

    fn owner(&self, name: &str) -> Result<Output, Error> {
        let owner = self.file_owner(name)?;
        let name = owner
            .name()
            .to_str()
            .expect("Unable to find file owner")
            .to_string();
        Ok(Output::Text(name))
    }

    fn group(&self, name: &str) -> Result<Output, Error> {
        let group = self.file_group(name)?;
        let name = group
            .name()
            .to_str()
            .expect("Unable to find file owner")
            .to_string();
        Ok(Output::Text(name))
    }

    fn linked_to(&self, name: &str) -> Result<Output, Error> {
        let followed_path = fs::read_link(name)?;
        let followed_path_name = followed_path.to_str().ok_or(StringError {
            string: "Path is invalid utf-8".to_string(),
        })?;
        Ok(Output::Text(followed_path_name.to_owned()))
    }

    fn is_readable(&self, name: &str, whom: Option<&Whom>) -> Result<Output, Error> {
        let mode = self.mode(name)?;
        let mode_octal = Output::to_i32(mode)?;
        let res = match whom {
            Some(w) => match *w {
                Whom::Owner => Output::Bool(mode_octal & 0o400 != 0),
                Whom::Group => Output::Bool(mode_octal & 0o040 != 0),
                Whom::Others => Output::Bool(mode_octal & 0o004 != 0),
                Whom::User(ref u) => self.is_readable_by_user(name, u)?,
            },
            None => Output::Bool(mode_octal & 0o444 != 0),
        };
        Ok(res)
    }

    fn is_writable(&self, name: &str, whom: Option<&Whom>) -> Result<Output, Error> {
        let mode = self.mode(name)?;
        let mode_octal = Output::to_i32(mode)?;
        let res = match whom {
            Some(w) => match *w {
                Whom::Owner => Output::Bool(mode_octal & 0o200 != 0),
                Whom::Group => Output::Bool(mode_octal & 0o020 != 0),
                Whom::Others => Output::Bool(mode_octal & 0o002 != 0),
                Whom::User(ref u) => self.is_writable_by_user(name, u)?,
            },
            None => Output::Bool(mode_octal & 0o222 != 0),
        };
        Ok(res)
    }

    fn md5sum(&self, name: &str) -> Result<Output, Error> {
        let content = self.file_content(name)?;
        let md5sum = format!("{:x}", md5::compute(content));
        Ok(Output::Text(md5sum))
    }

    fn sha256sum(&self, name: &str) -> Result<Output, Error> {
        let mut hasher = Sha256::new();
        let content = self.file_content(name)?;
        hasher.update(&content);
        let output = hasher.finalize();
        Ok(Output::Text(format!("{:x}", output)))
    }

    fn selinux_label(&self, _: &str) -> Result<Output, Error> {
        unimplemented!()
    }

    fn size(&self, name: &str) -> Result<Output, Error> {
        let file = fs::File::open(name)?;
        let raw_fd: RawFd = file.as_raw_fd();
        let file_stat = nix::sys::stat::fstat(raw_fd)?;
        let size = file_stat.st_size;
        Ok(Output::I64(size))
    }

    fn box_clone(&self) -> Box<dyn InlineProvider> {
        Box::new((*self).clone())
    }
}
