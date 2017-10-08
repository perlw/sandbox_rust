use super::context::gl;

pub type ShaderHandle = u32;

pub struct Shader {
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn new(handle: ShaderHandle) -> Self {
        Self { handle }
    }

    pub fn activate(&self) {
        unsafe {
            // TODO: Save state in context
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
