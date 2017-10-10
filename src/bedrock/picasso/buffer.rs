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

#[derive(Clone, Copy)]
pub enum BufferType {
    Int,
    Float,
}

impl BufferType {
    pub fn to_gl(&self) -> u32 {
        match self {
            Int => gl::INT,
            Float => gl::FLOAT,
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

    pub fn new_buffer(&mut self, target: BufferTarget) -> BufferHandle {
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
                target,
            ),
        );
        handle
    }

    pub fn with_buffer<F>(&mut self, handle: BufferHandle, fun: F) -> bool
    where
        F: Fn(&mut Buffer),
    {
        self.gl_state.borrow_mut().bind_buffergroup(self.handle);
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
    target: BufferTarget,
}

impl Buffer {
    pub fn new(gl_state: Rc<RefCell<GlState>>, handle: BufferHandle, target: BufferTarget) -> Self {
        Self {
            gl_state,
            handle,
            target,
        }
    }

    pub fn set_data<T>(&mut self, data: &[T]) {
        self.gl_state.borrow_mut().bind_buffer(
            self.target,
            self.handle,
        );
        unsafe {
            gl::BufferData(
                self.target.to_gl(),
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *mut _,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn vertex_attrib(&self, attrib: u32, num_elems: u32, elem_type: BufferType) {
        self.gl_state.borrow_mut().bind_buffer(
            self.target,
            self.handle,
        );
        unsafe {
            gl::EnableVertexAttribArray(attrib);
            gl::VertexAttribPointer(
                attrib,
                num_elems as i32,
                elem_type.to_gl(),
                gl::FALSE,
                0,
                std::ptr::null(),
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
