#![allow(bad_style)]

extern crate libc;

extern "C" {
    #[link_name = "glfwInit"]
    pub fn Init() -> c_int;
}
