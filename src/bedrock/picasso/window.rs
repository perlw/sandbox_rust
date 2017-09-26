extern crate glfw_sys as glfw;

use std;
use std::ffi::{CStr, CString};
use libc::{c_char, c_int};

#[allow(unused)]
extern "C" fn error_callback(error: c_int, description: *const c_char) {
    unsafe {
        println!("ERROR {}", CStr::from_ptr(description).to_str().unwrap());
        if error == glfw::NOT_INITIALIZED {
            println!("YAS");
        }
    }
}

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

pub struct Window {
    raw_ptr: *mut glfw::Window,
}

impl Window {
    pub fn new() -> Window {
        let mut window = Window {
            raw_ptr: std::ptr::null_mut(),
        };

        unsafe {
            glfw::SetErrorCallback(error_callback);

            glfw::Init();

            // Print monitors
            println!("-=Monitors=-");
            let mut count: c_int = 0;
            let raw_monitors = glfw::GetMonitors(&mut count);
            let monitors = std::slice::from_raw_parts(raw_monitors, count as usize);
            for m in monitors.iter() {
                let name = glfw::GetMonitorName(*m);
                let mut xpos: c_int = 0;
                let mut ypos: c_int = 0;
                let mut width_mm: c_int = 0;
                let mut height_mm: c_int = 0;
                glfw::GetMonitorPos(*m, &mut xpos, &mut ypos);
                glfw::GetMonitorPhysicalSize(*m, &mut width_mm, &mut height_mm);

                println!(
                    "{} @ {}x{} size: {}mm {}mm",
                    CStr::from_ptr(name).to_str().unwrap(),
                    xpos,
                    ypos,
                    width_mm,
                    height_mm
                );

                // Print Vidmodes
                let mut count_modes: c_int = 0;
                let raw_modes = glfw::GetVideoModes(*m, &mut count_modes);
                let modes = std::slice::from_raw_parts(raw_modes, count_modes as usize);
                for v in modes.iter() {
                    println!("\t{}x{}@{}hz", v.width, v.height, v.refreshRate);
                }
            }

            glfw::WindowHint(glfw::RESIZABLE, glfw::FALSE);
            window.raw_ptr = glfw::CreateWindow(
                640,
                480,
                CString::new("test title").unwrap().as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            glfw::SetWindowPosCallback(window.raw_ptr, window_pos_callback);
            glfw::SetKeyCallback(window.raw_ptr, key_callback);
            glfw::MakeContextCurrent(window.raw_ptr);

            let cursor = glfw::CreateStandardCursor(glfw::CROSSHAIR_CURSOR);
            glfw::SetCursor(window.raw_ptr, cursor);
        }

        window
    }

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
            glfw::Terminate();
        }
    }
}
