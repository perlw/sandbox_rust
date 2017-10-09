extern crate glfw_sys as glfw;
extern crate libc;

use std;
use std::ffi::{CStr, CString};
use self::libc::{c_float, c_int};
use std::ptr;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::shader::{ShaderHandle, Shader};
use super::buffer::{BufferGroupHandle, BufferHandle, BufferGroup, BufferType};

#[allow(unused)]
pub mod gl {
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
    context.debug_callback.expect("missing debug callback")(msg);
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

    pub fn create(&mut self) -> Rc<RefCell<Context>> {
        let context = Rc::new(RefCell::new(Context {
            debug_callback: self.debug_callback,
            shader_handles: HashMap::new(),
            buffergroup_handles: HashMap::new(),
            gl_state: Rc::new(RefCell::new(GlState::new())),
        }));

        unsafe {
            gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

            if self.debug_callback.is_some() {
                // Check for extension first
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                gl::DebugMessageCallback(debug_callback, context.as_ptr() as *mut _);
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

        context
    }
}

pub struct Context {
    pub debug_callback: Option<DebugFn>,
    pub shader_handles: HashMap<ShaderHandle, Shader>,
    pub buffergroup_handles: HashMap<BufferGroupHandle, BufferGroup>,
    gl_state: Rc<RefCell<GlState>>,
}

impl Context {
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn compile_shader(&self, source: &Vec<u8>, shader_type: u32) -> Result<u32, String> {
        let handle = unsafe { gl::CreateShader(shader_type) } as u32;

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

                gl::DeleteShader(handle);
            }

            return Err(String::from_utf8(buf).expect(
                "Shader compile: Failed to get shader err log",
            ));
        }

        Ok(handle)
    }

    fn attach_shaders(&self, vert_handle: u32, frag_handle: u32) -> Result<ShaderHandle, String> {
        let handle = unsafe { gl::CreateProgram() } as ShaderHandle;

        unsafe {
            gl::AttachShader(handle, vert_handle);
            gl::AttachShader(handle, frag_handle);
            gl::LinkProgram(handle);

            gl::DetachShader(handle, vert_handle);
            gl::DetachShader(handle, frag_handle);
            gl::DeleteShader(vert_handle);
            gl::DeleteShader(frag_handle);
        }

        let mut status = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut status);
        }
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut len);
            }
            let mut buf = Vec::<u8>::with_capacity(len as usize);
            unsafe {
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(handle, len, ptr::null_mut(), buf.as_mut_ptr() as *mut i8);

                gl::DeleteProgram(handle);
            }

            return Err(String::from_utf8(buf).expect(
                "Program attach: Failed to get program err log",
            ));
        }

        Ok(handle)
    }

    pub fn new_shader(
        &mut self,
        vert_source: &Vec<u8>,
        frag_source: &Vec<u8>,
    ) -> Option<ShaderHandle> {
        let vert = self.compile_shader(vert_source, gl::VERTEX_SHADER);
        let frag = self.compile_shader(frag_source, gl::FRAGMENT_SHADER);

        if vert.is_err() || frag.is_err() {
            return None;
        }

        match self.attach_shaders(vert.unwrap(), frag.unwrap()) {
            Ok(handle) => {
                self.shader_handles.insert(
                    handle,
                    Shader::new(
                        self.gl_state.clone(),
                        handle,
                    ),
                );
                Some(handle)
            }
            _ => None,
        }
    }

    pub fn new_buffergroup(&mut self) -> BufferGroupHandle {
        let mut handle = 0 as BufferGroupHandle;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
        }

        self.buffergroup_handles.insert(
            handle,
            BufferGroup::new(
                self.gl_state.clone(),
                handle,
            ),
        );
        handle
    }

    pub fn with_shader<F>(&mut self, handle: ShaderHandle, fun: F) -> bool
    where
        F: Fn(&mut Shader),
    {
        match self.shader_handles.get_mut(&handle) {
            Some(shader) => {
                self.gl_state.borrow_mut().bind_shader(handle);
                fun(shader);
                true
            }
            None => false,
        }
    }

    pub fn with_buffergroup<F>(&mut self, handle: BufferGroupHandle, fun: F) -> bool
    where
        F: Fn(&mut BufferGroup),
    {
        match self.buffergroup_handles.get_mut(&handle) {
            Some(buffergroup) => {
                self.gl_state.borrow_mut().bind_buffergroup(handle);
                fun(buffergroup);
                true
            }
            None => false,
        }
    }

    pub fn with_shader_and_buffergroup<F>(
        &mut self,
        shader_handle: ShaderHandle,
        buffergroup_handle: BufferGroupHandle,
        fun: F,
    ) -> bool
    where
        F: Fn(&mut Shader, &mut BufferGroup),
    {
        let shader = self.shader_handles.get_mut(&shader_handle);
        let buffergroup = self.buffergroup_handles.get_mut(&buffergroup_handle);

        if shader.is_some() && buffergroup.is_some() {
            self.gl_state.borrow_mut().bind_shader(shader_handle);
            self.gl_state.borrow_mut().bind_buffergroup(
                buffergroup_handle,
            );
            fun(shader.unwrap(), buffergroup.unwrap());
            return true;
        }

        false
    }
}

pub struct GlState {
    shader: ShaderHandle,
    buffergroup: BufferGroupHandle,
    buffer: BufferHandle,
}

impl GlState {
    pub fn new() -> Self {
        Self {
            shader: 0,
            buffergroup: 0,
            buffer: 0,
        }
    }

    pub fn bind_shader(&mut self, handle: ShaderHandle) {
        if self.shader != handle {
            unsafe {
                gl::UseProgram(handle);
            }
            self.shader = handle;
        }
    }

    pub fn bind_buffergroup(&mut self, handle: BufferGroupHandle) {
        if self.buffergroup != handle {
            unsafe {
                gl::BindVertexArray(handle);
            }
            self.buffergroup = handle;
        }
    }

    pub fn bind_buffer(&mut self, buffer_type: BufferType, handle: BufferHandle) {
        if self.buffer != handle {
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            }
            self.buffer = handle;
        }
    }
}
