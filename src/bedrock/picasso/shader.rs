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
