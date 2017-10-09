use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::ffi::CString;

use super::context::{gl, GlState};
use super::buffer::BufferType;

pub type ShaderHandle = u32;

pub struct Shader {
    gl_state: Rc<RefCell<GlState>>,
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: ShaderHandle) -> Self {
        Self { gl_state, handle }
    }

    pub fn get_attrib_location(&self, name: &str) -> i32 {
        unsafe { gl::GetAttribLocation(self.handle, CString::new(name).unwrap().as_ptr()) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
