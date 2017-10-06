use super::context::{gl, ShaderHandle};

pub struct Shader {
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
