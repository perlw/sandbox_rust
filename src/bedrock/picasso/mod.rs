extern crate libc;
extern crate glfw_sys as glfw;

use std;
use std::ffi::CStr;
use self::libc::{c_char, c_int};

pub mod window;
pub mod canvas;

use self::window::{Window, WindowConfig};
use self::canvas::{Canvas, CanvasConfig};

#[allow(unused)]
extern "C" fn error_callback(error: c_int, description: *const c_char) {
    unsafe {
        println!("ERROR {}", CStr::from_ptr(description).to_str().unwrap());
        if error == glfw::NOT_INITIALIZED {
            println!("YAS");
        }
    }
}

pub struct Picasso {}

impl Picasso {
    pub fn new() -> Picasso {
        unsafe {
            glfw::SetErrorCallback(error_callback);

            glfw::Init();
            glfw::SwapInterval(0);

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
        }

        Picasso {}
    }

    pub fn poll_events(&self) {
        unsafe {
            glfw::PollEvents();
        }
    }

    pub fn new_window(&self) -> WindowConfig {
        WindowConfig {
            title: "picasso".into(),
            width: 640,
            height: 480,
            resizable: true,
            ogl_major: 1,
            ogl_minor: 0,
            ogl_debug: false,
        }
    }
}

impl Drop for Picasso {
    fn drop(&mut self) {
        unsafe {
            glfw::Terminate();
        }
    }
}
