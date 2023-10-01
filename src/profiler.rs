use crate::implementations::os::*;

pub trait Profiler {
    fn setup(&mut self);
}
