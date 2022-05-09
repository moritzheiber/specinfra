use crate::backend::Backend;
use crate::provider::error;
use crate::provider::package::PackageProvider;
use crate::provider::Output;

pub struct Package<'a> {
    name: &'static str,
    version: Option<&'static str>,
    backend: &'a dyn Backend,
    provider: &'a PackageProvider,
}

impl<'a> Package<'a> {
    pub fn new(
        n: &'static str,
        v: Option<&'static str>,
        b: &'a dyn Backend,
        p: &'a PackageProvider,
    ) -> Package<'a> {
        Package {
            name: n,
            version: v,
            backend: b,
            provider: p,
        }
    }

    pub fn is_installed(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_installed(self.name, self.version))
            .and_then(Output::to_bool)
    }

    pub fn version(&self) -> Result<String, error::Error> {
        self.backend
            .handle(self.provider.version(self.name, self.version))
            .and_then(Output::to_string)
    }

    pub fn remove(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.remove(self.name, self.version))
            .and_then(Output::to_bool)
    }

    pub fn install(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.install(self.name, self.version))
            .and_then(Output::to_bool)
    }
}
