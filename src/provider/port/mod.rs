use crate::provider::port::inline::InlineProvider;
use crate::provider::port::shell::ShellProvider;
use crate::provider::HandleFunc;

pub struct PortProvider {
    pub inline: Box<dyn InlineProvider>,
    pub shell: Box<dyn ShellProvider>,
}

impl PortProvider {
    pub fn is_listening(&self, number: usize) -> Box<HandleFunc> {
        let i = self.inline.clone();
        let s = self.shell.clone();
        Box::new(HandleFunc {
            inline: Box::new(move || i.is_listening(number)),
            shell: Box::new(move |b| s.is_listening(number, b)),
        })
    }
}

pub mod inline;
pub mod shell;
