extern crate glfw_sys as glfw;
extern crate libc;

use std;
use std::ffi::CString;
use self::libc::c_int;
use std::rc::Rc;
use std::cell::RefCell;

use super::context::{Context, ContextConfig};

pub type KeyFn = fn(window: &mut Window, key: i32, scancode: i32);

#[allow(unused)]
extern "C" fn window_pos_callback(raw_window: *mut glfw::Window, xpos: c_int, ypos: c_int) {
    let window = unsafe { &mut *(glfw::GetWindowUserPointer(raw_window) as *mut Window) };

    println!("WindowPos: {}x{}", xpos, ypos);
}

#[allow(unused)]
extern "C" fn key_callback(
    raw_window: *mut glfw::Window,
    key: c_int,
    scancode: c_int,
    action: c_int,
    mods: c_int,
) {
    // Hilariously unsafe
    let window = unsafe { &mut *(glfw::GetWindowUserPointer(raw_window) as *mut Window) };
    (window.key_fn)(window, key as i32, scancode as i32);
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

    pub fn create(&mut self) -> Result<Box<Window>, bool> {
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
        }

        let raw_ptr = unsafe {
            glfw::CreateWindow(
                self.width as c_int,
                self.height as c_int,
                CString::new(self.title.as_str()).unwrap().as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        unsafe {
            glfw::MakeContextCurrent(raw_ptr);
        }

        let mut window = Box::new(Window {
            raw_ptr,
            context: self.context_config.create(),
            width: self.width,
            height: self.height,
            key_fn: |_a, _b, _c|{},
        });

        unsafe {
            glfw::SetWindowUserPointer(raw_ptr, &mut *window as *mut _ as *mut _);
            glfw::SetWindowPosCallback(raw_ptr, window_pos_callback);
            glfw::SetKeyCallback(raw_ptr, key_callback);
            //glfw::SwapInterval(0);
        }

        Ok(window)
    }
}

pub struct Window {
    raw_ptr: *mut glfw::Window,
    context: Rc<RefCell<Context>>,
    width: u32,
    height: u32,
    key_fn: KeyFn,
}

impl Window {
    pub fn keyboard_callback(&mut self, fun: KeyFn) {
        self.key_fn = fun;
    }

    pub fn make_context_current(&self) {
        unsafe {
            glfw::MakeContextCurrent(self.raw_ptr);
        }
    }

    pub fn should_close(&self) -> bool {
        unsafe { (glfw::WindowShouldClose(self.raw_ptr) == glfw::TRUE) }
    }

    pub fn set_should_close(&mut self, flag: bool) {
        unsafe {
            glfw::SetWindowShouldClose(self.raw_ptr, if flag { glfw::TRUE } else { glfw::FALSE });
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glfw::SwapBuffers(self.raw_ptr);
        }
    }

    pub fn with_context<T, F>(&mut self, fun: F) -> T
    where
        F: Fn(&mut Context) -> T,
    {
        self.make_context_current();

        let ctxt = self.context.clone();
        let result = fun(&mut *ctxt.borrow_mut());
        return result;
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfw::DestroyWindow(self.raw_ptr);
        }
    }
}
