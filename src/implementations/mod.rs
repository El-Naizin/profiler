#[cfg_attr(target_os = "macos", path = "mac.rs")]
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod os;