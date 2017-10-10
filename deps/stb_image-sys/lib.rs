#![allow(bad_style)]

extern crate libc;

use libc::{c_int, c_void};

extern "C" {
    #[link_name = "stbi_load_from_memory"]
    pub fn load_from_memory(
        buffer: *const c_void,
        len: c_int,
        x: *mut c_int,
        y: *mut c_int,
        channels_in_file: *mut c_int,
        desired_channels: c_int,
    ) -> *mut c_void;
}
