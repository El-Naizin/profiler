use profiler::Profiler;
use crate::profiler;

pub struct LinuxProfiler {
}

impl Profiler for LinuxProfiler {
    fn setup(&mut self) {
        println!("Linux profiling setup")
    }
}

pub fn get_profiler() -> impl Profiler{
    LinuxProfiler{}
}