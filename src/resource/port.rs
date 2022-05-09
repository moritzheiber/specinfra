use crate::backend::Backend;
use crate::provider::error;
use crate::provider::port::PortProvider;
use crate::provider::Output;

pub struct Port<'a> {
    number: usize,
    backend: &'a dyn Backend,
    provider: &'a PortProvider,
}

impl<'a> Port<'a> {
    pub fn new(n: usize, b: &'a dyn Backend, p: &'a PortProvider) -> Port<'a> {
        Port {
            number: n,
            backend: b,
            provider: p,
        }
    }

    pub fn is_listening(&self) -> Result<bool, error::Error> {
        self.backend
            .handle(self.provider.is_listening(self.number))
            .and_then(Output::to_bool)
    }
}
