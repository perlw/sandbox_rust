extern crate glfw_sys as glfw;
extern crate libc;

#[allow(unused)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std;
use std::ffi::CString;
use self::libc::c_int;

use super::context::{Context, ContextConfig};

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
    pub ogl_major: u32,
    pub ogl_minor: u32,
    pub ogl_debug: bool,
    pub context_config: ContextConfig,
}

impl WindowConfig {
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn resizable(&mut self, flag: bool) -> &mut Self {
        self.resizable = flag;
        self
    }

    pub fn opengl_context_version(&mut self, major: u32, minor: u32) -> &mut Self {
        self.ogl_major = major;
        self.ogl_minor = minor;
        self
    }

    pub fn opengl_context_debug(&mut self, flag: bool) -> &mut Self {
        self.ogl_debug = flag;
        self
    }

    pub fn with_context_config<F>(&mut self, fun: F) -> &mut Self
    where
        F: Fn(&mut ContextConfig),
    {
        fun(&mut self.context_config);
        self
    }

    pub fn create(&mut self) -> Result<Window, bool> {
        let mut window = Window {
            raw_ptr: std::ptr::null_mut(),
            context: None,
            width: 640,
            height: 480,
        };

        unsafe {
            glfw::DefaultWindowHints();
            glfw::WindowHint(glfw::CONTEXT_VERSION_MAJOR, self.ogl_major as c_int);
            glfw::WindowHint(glfw::CONTEXT_VERSION_MINOR, self.ogl_minor as c_int);
            glfw::WindowHint(glfw::OPENGL_PROFILE, glfw::OPENGL_CORE_PROFILE);
            glfw::WindowHint(
                glfw::OPENGL_DEBUG_CONTEXT,
                if self.ogl_debug {
                    glfw::TRUE
                } else {
                    glfw::FALSE
                },
            );
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

            window.make_context_current();
            window.context = match self.context_config.create() {
                Ok(context) => Some(Box::new(context)),
                Err(_) => None,
            };
        }

        Ok(window)
    }
}

pub struct Window {
    raw_ptr: *mut glfw::Window,
    context: Option<Box<Context>>,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn make_context_current(&self) {
        unsafe {
            glfw::MakeContextCurrent(self.raw_ptr);
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { (glfw::WindowShouldClose(self.raw_ptr) == glfw::TRUE) }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glfw::SwapBuffers(self.raw_ptr);
        }
    }

    pub fn with_context<T, F>(&mut self, fun: F) -> T
    where
        F: Fn(&mut Box<Context>) -> T,
    {
        self.make_context_current();
        fun(&mut self.context.as_mut().expect("Must have a context"))
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfw::DestroyWindow(self.raw_ptr);
        }
    }
}
