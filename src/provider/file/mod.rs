use crate::provider::file::inline::InlineProvider;
use crate::provider::file::shell::ShellProvider;
use crate::provider::HandleFunc;

pub enum Whom {
    Owner,
    Group,
    Others,
    User(String),
}

pub struct FileProvider {
    pub inline: Box<dyn InlineProvider>,
    pub shell: Box<dyn ShellProvider>,
}

impl FileProvider {
    pub fn new(i: Box<dyn InlineProvider>, s: Box<dyn ShellProvider>) -> FileProvider {
        FileProvider {
            inline: i,
            shell: s,
        }
    }

    pub fn mode(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.mode(name)),
            shell: Box::new(move |b| s.mode(name, b)),
        })
    }

    pub fn size(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.size(name)),
            shell: Box::new(move |b| s.size(name, b)),
        })
    }

    pub fn is_file(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_file(name)),
            shell: Box::new(move |b| s.is_file(name, b)),
        })
    }

    pub fn is_directory(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_directory(name)),
            shell: Box::new(move |b| s.is_directory(name, b)),
        })
    }

    pub fn is_block_device(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_block_device(name)),
            shell: Box::new(move |b| s.is_block_device(name, b)),
        })
    }

    pub fn is_character_device(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_character_device(name)),
            shell: Box::new(move |b| s.is_character_device(name, b)),
        })
    }

    pub fn is_pipe(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_pipe(name)),
            shell: Box::new(move |b| s.is_pipe(name, b)),
        })
    }

    pub fn is_socket(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_socket(name)),
            shell: Box::new(move |b| s.is_socket(name, b)),
        })
    }

    pub fn is_symlink(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_symlink(name)),
            shell: Box::new(move |b| s.is_symlink(name, b)),
        })
    }

    pub fn exist(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.exist(name)),
            shell: Box::new(move |b| s.exist(name, b)),
        })
    }

    pub fn contents(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.contents(name)),
            shell: Box::new(move |b| s.contents(name, b)),
        })
    }

    pub fn owner(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.owner(name)),
            shell: Box::new(move |b| s.owner(name, b)),
        })
    }

    pub fn group(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.group(name)),
            shell: Box::new(move |b| s.group(name, b)),
        })
    }

    pub fn linked_to(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.linked_to(name)),
            shell: Box::new(move |b| s.linked_to(name, b)),
        })
    }

    pub fn is_readable(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, None)),
            shell: Box::new(move |b| s.is_readable(name, None, b)),
        })
    }

    pub fn is_readable_by_owner(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, Some(&Whom::Owner))),
            shell: Box::new(move |b| s.is_readable(name, Some(&Whom::Owner), b)),
        })
    }

    pub fn is_readable_by_group(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, Some(&Whom::Group))),
            shell: Box::new(move |b| s.is_readable(name, Some(&Whom::Group), b)),
        })
    }

    pub fn is_readable_by_others(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, Some(&Whom::Others))),
            shell: Box::new(move |b| s.is_readable(name, Some(&Whom::Others), b)),
        })
    }

    pub fn is_readable_by_user(&self, name: &'static str, user: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_readable(name, Some(&Whom::User(user.to_string())))),
            shell: Box::new(move |b| s.is_readable(name, Some(&Whom::User(user.to_string())), b)),
        })
    }

    pub fn is_writable(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_writable(name, None)),
            shell: Box::new(move |b| s.is_writable(name, None, b)),
        })
    }

    pub fn is_writable_by_owner(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_writable(name, Some(&Whom::Owner))),
            shell: Box::new(move |b| s.is_writable(name, Some(&Whom::Owner), b)),
        })
    }

    pub fn is_writable_by_group(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_writable(name, Some(&Whom::Group))),
            shell: Box::new(move |b| s.is_writable(name, Some(&Whom::Group), b)),
        })
    }

    pub fn is_writable_by_others(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_writable(name, Some(&Whom::Others))),
            shell: Box::new(move |b| s.is_writable(name, Some(&Whom::Others), b)),
        })
    }

    pub fn is_writable_by_user(&self, name: &'static str, user: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_writable(name, Some(&Whom::User(user.to_string())))),
            shell: Box::new(move |b| s.is_writable(name, Some(&Whom::User(user.to_string())), b)),
        })
    }

    pub fn md5sum(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.md5sum(name)),
            shell: Box::new(move |b| s.md5sum(name, b)),
        })
    }

    pub fn sha256sum(&self, name: &'static str) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.sha256sum(name)),
            shell: Box::new(move |b| s.sha256sum(name, b)),
        })
    }
}

pub mod inline;
pub mod shell;
