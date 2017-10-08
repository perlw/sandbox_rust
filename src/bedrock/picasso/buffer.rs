use std;
use std::collections::HashMap;

use super::context::gl;

pub type BufferGroupHandle = u32;
pub type BufferHandle = u32;

pub struct BufferGroup {
    handle: BufferGroupHandle,
    buffers: HashMap<BufferHandle, Buffer>,
}

impl BufferGroup {
    pub fn new(handle: BufferGroupHandle) -> Self {
        Self {
            handle,
            buffers: HashMap::new(),
        }
    }

    pub fn new_buffer(&mut self) -> BufferHandle {
        let mut handle = 0 as BufferHandle;

        unsafe {
            // TODO: Save state in context
            gl::BindVertexArray(handle);

            gl::GenBuffers(1, &mut handle);
            // TODO: Save state in context
            //gl::BindBuffer(target, handle);
        }

        self.buffers.insert(handle, Buffer::new(handle));
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
    handle: BufferHandle,
}

impl Buffer {
    pub fn new(handle: BufferHandle) -> Self {
        Self { handle }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle);
        }
    }
}
