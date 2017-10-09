use std;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::context::{gl, GlState};

pub enum BufferType {
    VertexBuffer,
    CoordBuffer,
}

pub type BufferGroupHandle = u32;
pub type BufferHandle = u32;

pub struct BufferGroup {
    gl_state: Rc<RefCell<GlState>>,
    handle: BufferGroupHandle,
    buffers: HashMap<BufferHandle, Buffer>,
}

impl BufferGroup {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: BufferGroupHandle) -> Self {
        Self {
            gl_state,
            handle,
            buffers: HashMap::new(),
        }
    }

    pub fn new_buffer(&mut self) -> BufferHandle {
        let mut handle = 0 as BufferHandle;

        self.gl_state.borrow_mut().bind_buffergroup(handle);
        unsafe {
            gl::GenBuffers(1, &mut handle);
        }

        self.buffers.insert(
            handle,
            Buffer::new(self.gl_state.clone(), handle),
        );
        handle
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

    pub fn set_data<T>(&mut self, buffer_type: BufferType, data: Vec<T>) {
        self.gl_state.borrow_mut().bind_buffer(
            buffer_type,
            self.handle,
        );
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle);
        }
    }
}
