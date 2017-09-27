extern crate glfw_sys as glfw;

use std;
use std::ffi::CString;
use libc::c_int;

#[allow(unused)]
extern "C" fn window_pos_callback(window: *mut glfw::Window, xpos: c_int, ypos: c_int) {
    println!("WindowPos: {}x{}", xpos, ypos);
}

#[allow(unused)]
extern "C" fn key_callback(
    window: *mut glfw::Window,
    key: c_int,
    scancode: c_int,
    action: c_int,
    mods: c_int,
) {
    if key == glfw::KEY_ESCAPE {
        unsafe {
            glfw::SetWindowShouldClose(window, glfw::TRUE);
        }
    }
}

pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl WindowConfig {
    pub fn title(&mut self, val: String) -> &mut Self {
        self.title = val;
        self
    }

    pub fn width(&mut self, val: u32) -> &mut Self {
        self.width = val;
        self
    }

    pub fn height(&mut self, val: u32) -> &mut Self {
        self.height = val;
        self
    }

    pub fn resizable(&mut self, flag: bool) -> &mut Self {
        self.resizable = flag;
        self
    }

    pub fn create(&self) -> Result<Window, bool> {
        let mut window = Window { raw_ptr: std::ptr::null_mut() };

        unsafe {
            glfw::DefaultWindowHints();
            glfw::WindowHint(glfw::CONTEXT_VERSION_MAJOR, 4);
            glfw::WindowHint(glfw::CONTEXT_VERSION_MINOR, 0);
            glfw::WindowHint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
            /*
            glfw::WindowHint(glfw::OPENGL_DEBUG_CONTEXT, ...);
            glEnable(GL_DEBUG_OUTPUT);
            glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB);
            glDebugMessageCallback((GLDEBUGPROC)debug_callback, NULL);
            */

            glfw::WindowHint(
                glfw::RESIZABLE,
                if self.resizable {
                    glfw::TRUE
                } else {
                    glfw::FALSE
                },
            );
            window.raw_ptr = glfw::CreateWindow(
                self.width as c_int,
                self.height as c_int,
                CString::new(self.title.as_str()).unwrap().as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            glfw::SetWindowPosCallback(window.raw_ptr, window_pos_callback);
            glfw::SetKeyCallback(window.raw_ptr, key_callback);
            glfw::MakeContextCurrent(window.raw_ptr);
        }

        Ok(window)
    }
}

pub struct Window {
    raw_ptr: *mut glfw::Window,
}

impl Window {
    pub fn should_close(&self) -> bool {
        unsafe { (glfw::WindowShouldClose(self.raw_ptr) == glfw::TRUE) }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glfw::SwapBuffers(self.raw_ptr);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfw::DestroyWindow(self.raw_ptr);
        }
    }
}
