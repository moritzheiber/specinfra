use crate::provider::package::inline::InlineProvider;
use crate::provider::package::shell::ShellProvider;
use crate::provider::HandleFunc;

pub struct PackageProvider {
    pub inline: Box<dyn InlineProvider>,
    pub shell: Box<dyn ShellProvider>,
}

impl PackageProvider {
    pub fn is_installed(
        &self,
        name: &'static str,
        version: Option<&'static str>,
    ) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_installed(name, version)),
            shell: Box::new(move |b| s.is_installed(name, version, b)),
        })
    }

    pub fn version(&self, name: &'static str, version: Option<&'static str>) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.version(name, version)),
            shell: Box::new(move |b| s.version(name, version, b)),
        })
    }

    pub fn remove(&self, name: &'static str, version: Option<&'static str>) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.remove(name, version)),
            shell: Box::new(move |b| s.remove(name, version, b)),
        })
    }

    pub fn install(&self, name: &'static str, version: Option<&'static str>) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.install(name, version)),
            shell: Box::new(move |b| s.install(name, version, b)),
        })
    }
}

pub mod inline;
pub mod shell;
