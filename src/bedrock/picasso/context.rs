extern crate libc;
extern crate glfw_sys as glfw;

use std;
use std::ffi::{CStr, CString};
use self::libc::{c_int, c_float};
use std::ptr;

use super::shader;

#[allow(unused)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub type DebugFn = fn(message: String);

#[allow(unused)]
extern "system" fn debug_callback(
    source: gl::types::GLenum,
    gltype: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    user_param: *mut std::os::raw::c_void,
) {
    let msg = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };
    let context = unsafe { &mut *(user_param as *mut Context) };
    (context.debug_callback.expect("missing debug callback"))(msg);
}

pub struct ContextConfig {
    pub debug_callback: Option<DebugFn>,
    pub viewport_x: u32,
    pub viewport_y: u32,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub clear_color_r: f32,
    pub clear_color_g: f32,
    pub clear_color_b: f32,
    pub clear_color_a: f32,
}

impl ContextConfig {
    pub fn debug(&mut self, fun: DebugFn) -> &mut Self {
        self.debug_callback = Some(fun);
        self
    }

    pub fn viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> &mut Self {
        self.viewport_x = x;
        self.viewport_y = y;
        self.viewport_width = width;
        self.viewport_height = height;
        self
    }

    pub fn clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.clear_color_r = r;
        self.clear_color_g = g;
        self.clear_color_b = b;
        self.clear_color_a = a;
        self
    }

    pub fn create(&mut self) -> Result<Context, bool> {
        let mut context = Context {
            debug_callback: self.debug_callback,
        };

        unsafe {
            gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

            if self.debug_callback.is_some() {
                // Check for extension first
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                gl::DebugMessageCallback(debug_callback, &mut context as *mut _ as *mut _);
            }

            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearDepth(1.0);
            gl::DepthFunc(gl::LESS);

            gl::Viewport(
                self.viewport_x as c_int,
                self.viewport_y as c_int,
                self.viewport_width as c_int,
                self.viewport_height as c_int,
            );
            gl::ClearColor(
                self.clear_color_r as c_float,
                self.clear_color_g as c_float,
                self.clear_color_b as c_float,
                self.clear_color_a as c_float,
            );
        }

        Ok(context)
    }
}

pub type Handle = u32;

pub struct Context {
    pub debug_callback: Option<DebugFn>,
}

impl Context {
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn create_shader(&self, source: &Vec<u8>) -> Result<shader::Shader, String> {
        let handle = unsafe { gl::CreateShader(gl::VERTEX_SHADER) } as Handle;

        unsafe {
            let c_source = CString::new(&source[..]).unwrap();
            gl::ShaderSource(handle, 1, &c_source.as_ptr(), ptr::null());
            gl::CompileShader(handle);
        }

        let mut status = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status);
        }
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buf = Vec::<u8>::with_capacity(len as usize);
            unsafe {
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(handle, len, ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
            }

            return Err(String::from_utf8(buf).expect("Failed to get shader err log"));
        }

        Ok(shader::Shader{})
    }
}
