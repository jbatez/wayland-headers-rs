use core::ffi::{CStr, c_int};

pub const WAYLAND_VERSION_MAJOR: c_int = 1;
pub const WAYLAND_VERSION_MINOR: c_int = 18;
pub const WAYLAND_VERSION_MICRO: c_int = 0;
pub const WAYLAND_VERSION: &CStr = c"1.18.0";
