#![allow(dead_code)]
extern crate libc;
extern crate glfw_sys as glfw;

use std;
use std::ffi::CStr;
use self::libc::{c_char, c_int};

pub mod window;
pub mod context;
pub mod shader;

#[allow(unused_imports)]
use self::window::{Window, WindowConfig};
#[allow(unused_imports)]
use self::context::{Context, ContextConfig};
#[allow(unused_imports)]
use self::shader::Shader;

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
    pub fn new() -> Self {
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
            context_config: ContextConfig {
                debug_callback: None,
                viewport_x: 0,
                viewport_y: 0,
                viewport_width: 640,
                viewport_height: 480,
                clear_color_r: 0.0,
                clear_color_g: 0.0,
                clear_color_b: 0.0,
                clear_color_a: 0.0,
            },
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
