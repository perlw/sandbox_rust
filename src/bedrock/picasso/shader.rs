use std;
use std::rc::Rc;

use super::context::{Context, ShaderHandle};

#[allow(unused)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Shader {
    //pub context: Rc<context::Context>,
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }
}
