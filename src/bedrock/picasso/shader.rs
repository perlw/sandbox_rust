use std;
use std::rc::Rc;
use std::cell::RefCell;

use super::context::{gl, GlState};

pub type ShaderHandle = u32;

pub struct Shader {
    gl_state: Rc<RefCell<GlState>>,
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: ShaderHandle) -> Self {
        Self { gl_state, handle }
    }

    pub fn bind(&mut self) {
        self.gl_state.borrow_mut().bind_shader(self.handle);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
