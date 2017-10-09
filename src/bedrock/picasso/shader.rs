extern crate cgmath;

use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::ffi::CString;

use cgmath::prelude::*;

use super::context::{gl, GlState};
use super::buffer::BufferType;

pub type ShaderHandle = u32;
pub type ShaderAttrib = i32;
pub type ShaderUniform = i32;

pub enum ShaderUniformData {
    Int(i32),
    Float(f32),
    Ivec2(cgmath::Vector2<i32>),
    Ivec3(cgmath::Vector3<i32>),
    Ivec4(cgmath::Vector4<i32>),
    Fvec2(cgmath::Vector2<f32>),
    Fvec3(cgmath::Vector3<f32>),
    Fvec4(cgmath::Vector4<f32>),
    Mat4(cgmath::Matrix4<f32>),
}

pub struct Shader {
    gl_state: Rc<RefCell<GlState>>,
    pub handle: ShaderHandle,
}

impl Shader {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: ShaderHandle) -> Self {
        Self { gl_state, handle }
    }

    pub fn get_attrib_location(&self, name: &str) -> ShaderAttrib {
        self.gl_state.borrow_mut().bind_shader(self.handle);
        unsafe { gl::GetAttribLocation(self.handle, CString::new(name).unwrap().as_ptr()) }
    }

    pub fn get_uniform_location(&self, name: &str) -> ShaderUniform {
        self.gl_state.borrow_mut().bind_shader(self.handle);
        unsafe { gl::GetUniformLocation(self.handle, CString::new(name).unwrap().as_ptr()) }
    }

    pub fn set_uniform(&self, uniform: ShaderUniform, data: ShaderUniformData) {
        self.gl_state.borrow_mut().bind_shader(self.handle);
        match data {
            ShaderUniformData::Mat4(matrix) => unsafe {
                gl::ProgramUniformMatrix4fv(self.handle, uniform, 1, gl::FALSE, matrix.as_ptr());
            },
            _ => unreachable!(),
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
