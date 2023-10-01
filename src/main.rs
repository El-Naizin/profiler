// use libloading;
// use libloading::{Library, Symbol};
//
// /*
//  * Kperf dynamic library loading
// #define lib_path_kperf "/System/Library/PrivateFrameworks/kperf.framework/kperf"
// #define lib_path_kperfdata                                                     \
//   "/System/Library/PrivateFrameworks/kperfdata.framework/kperfdata"
// */
//
// type AddFunc = unsafe fn(isize, isize) -> isize;
//
// /// Get the version of KPC that's being run.
// /// @return See `PMU version constants` above.
// /// @details sysctl get(kpc.pmu_version)
// type kpc_pmu_version = unsafe fn() -> u32;
//
// fn main() {
//     let kperf_lib_path = "/System/Library/PrivateFrameworks/kperf.framework/kperf";
//     let kperf_data_lib_path = "/System/Library/PrivateFrameworks/kperfdata.framework/kperfdata";
//
//     unsafe {
//         let lib_handle_kperf = Library::new(kperf_lib_path).unwrap();
//         let lib_handle_kperf_data = Library::new(kperf_data_lib_path).unwrap();
//         // let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
//         let func: Symbol<kpc_pmu_version> = lib_handle_kperf.get(b"kpc_pmu_version").unwrap();
//
//         // let answer = func(1, 2);
//         println!("Hello, world 2! {}", func());
//     }
// }

use profiler::implementations::os::get_profiler;
use profiler::profiler::Profiler;

fn main() {
    let mut a = get_profiler();
    a.setup();
}