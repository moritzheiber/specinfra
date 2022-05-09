use crate::platform::base_platform::BasePlatform;
use crate::platform::bsd;
use crate::platform::linux;
use crate::platform::platform::Platform;

pub struct Platforms {
    curr: usize,
    base_platforms: Vec<Box<dyn BasePlatform<Item = Box<dyn Platform>>>>,
}

impl Platforms {
    pub fn new() -> Platforms {
        let mut p: Vec<Box<dyn BasePlatform<Item = Box<dyn Platform>>>> = Vec::new();
        p.push(Box::new(bsd::Bsd::new()));
        p.push(Box::new(linux::Linux::new()));

        Platforms {
            curr: 0,
            base_platforms: p,
        }
    }
}

impl Iterator for Platforms {
    type Item = Box<dyn Platform>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.base_platforms.len() {
            match self.base_platforms[self.curr].next() {
                Some(x) => Some(x),
                None => {
                    self.curr += 1;
                    self.next()
                }
            }
        } else {
            None
        }
    }
}
