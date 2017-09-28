use std;
use std::ffi::CStr;
use std::ffi::CString;

extern crate glfw_sys as glfw;
#[allow(unused)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use super::window::Window;

extern "system" fn debug_callback(
    source: gl::types::GLenum,
    gltype: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    userParam: *mut std::os::raw::c_void,
) {
    unsafe {
        println!("GL ERR: {}", CStr::from_ptr(message).to_string_lossy());
    }
}

pub struct CanvasConfig<'a> {
    pub window: &'a Window,
}

impl<'a> CanvasConfig<'a> {
    pub fn create(&self) -> Result<Canvas, bool> {
        self.window.make_context_current();

        unsafe {
            gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(debug_callback, std::ptr::null());

            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearDepth(1.0);
            gl::DepthFunc(gl::LESS);
            gl::Viewport(0, 0, 640, 480);
            gl::ClearColor(0.5, 0.5, 1.0, 1.0);
        }

        Ok(Canvas {})
    }
}

pub struct Canvas {}

impl Canvas {
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
