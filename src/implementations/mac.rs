use profiler::Profiler;
use crate::profiler;

pub struct MacProfiler {
}

impl Profiler for MacProfiler {
    fn setup(&mut self) {
        println!("MacOS profiling setup")
    }
}

pub fn get_profiler() -> impl Profiler{
    MacProfiler{}
}
