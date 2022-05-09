use crate::backend::Backend;
use crate::platform::error::Error;
use crate::provider;

use std::fmt::Debug;
use std::result::Result;

// See https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-trait-object
pub trait Platform: PlatformClone + Debug {
    fn new() -> Self
    where
        Self: Sized;

    fn inline_detector(&self) -> Option<Box<dyn Platform>>;

    fn shell_detector(&self, _: &dyn Backend) -> Option<Box<dyn Platform>>;

    fn get_providers(&self) -> Result<Box<provider::Providers>, Error>;
}

pub trait PlatformClone {
    fn clone_box(&self) -> Box<dyn Platform>;
}

impl<T> PlatformClone for T
where
    T: 'static + Platform + Clone,
{
    fn clone_box(&self) -> Box<dyn Platform> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Platform> {
    fn clone(&self) -> Box<dyn Platform> {
        self.clone_box()
    }
}
