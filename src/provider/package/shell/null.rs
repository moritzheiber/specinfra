use super::ShellProvider;

#[derive(Clone, Debug)]
pub struct Null;

impl ShellProvider for Null {
    fn box_clone(&self) -> Box<dyn ShellProvider> {
        Box::new((*self).clone())
    }
}
