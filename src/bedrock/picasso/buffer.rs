use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::context::{gl, GlState};

#[derive(Clone, Copy)]
pub enum BufferTarget {
    ArrayBuffer,
}

impl BufferTarget {
    pub fn to_gl(&self) -> u32 {
        match self {
            ArrayBuffer => gl::ARRAY_BUFFER,
            _ => unreachable!(),
        }
    }
}

pub type BufferGroupHandle = u32;
pub type BufferHandle = u32;

pub struct BufferGroup {
    gl_state: Rc<RefCell<GlState>>,
    handle: BufferGroupHandle,
    buffer_handles: HashMap<BufferHandle, Buffer>,
}

impl BufferGroup {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: BufferGroupHandle) -> Self {
        Self {
            gl_state,
            handle,
            buffer_handles: HashMap::new(),
        }
    }

    pub fn new_buffer(&mut self) -> BufferHandle {
        let mut handle = 0 as BufferHandle;

        self.gl_state.borrow_mut().bind_buffergroup(handle);
        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        self.buffer_handles.insert(
            handle,
            Buffer::new(
                self.gl_state.clone(),
                handle,
            ),
        );
        handle
    }

    pub fn with_buffer<F>(&mut self, handle: BufferHandle, fun: F) -> bool
    where
        F: Fn(&mut Buffer),
    {
        match self.buffer_handles.get_mut(&handle) {
            Some(buffer) => {
                fun(buffer);
                true
            }
            None => false,
        }
    }

    pub fn draw(&self) {
        self.gl_state.borrow_mut().bind_buffergroup(self.handle);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl Drop for BufferGroup {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.handle);
        }
    }
}

pub struct Buffer {
    gl_state: Rc<RefCell<GlState>>,
    handle: BufferHandle,
}

impl Buffer {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: BufferHandle) -> Self {
        Self { gl_state, handle }
    }

    pub fn set_data<T>(&mut self, buffer_target: BufferTarget, mut data: Vec<T>) {
        self.gl_state.borrow_mut().bind_buffer(
            buffer_target,
            self.handle,
        );
        unsafe {
            gl::BufferData(
                buffer_target.to_gl(),
                0, /* INVALID */
                data.as_mut_ptr() as *mut i32 as *mut _,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle);
        }
    }
}
