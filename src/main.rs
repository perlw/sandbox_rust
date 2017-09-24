extern crate glfw_sys as glfw;
mod gl {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate libc;

use std::ffi::{CStr, CString};
use libc::{c_char, c_int};

extern fn error_callback(error: c_int, description: *const c_char) {
  unsafe {
    println!("ERROR {}", CStr::from_ptr(description).to_str().unwrap());
    if error == glfw::NOT_INITIALIZED {
      println!("YAS");
    }
  }
}

extern fn window_pos_callback(window: *mut glfw::Window, xpos: c_int, ypos: c_int) {
  println!("WindowPos: {}x{}", xpos, ypos);
}

extern fn key_callback(window: *mut glfw::Window, key: c_int, scancode: c_int, action: c_int, mods: c_int) {
  if key == glfw::KEY_ESCAPE {
    unsafe {
      glfw::SetWindowShouldClose(window, glfw::TRUE);
    }
  }
}

fn main() {
  unsafe {
    glfw::SetErrorCallback(error_callback);
    // Force error to trigger
    glfw::PollEvents();

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

      println!("{} @ {}x{} size: {}mm {}mm", CStr::from_ptr(name).to_str().unwrap(), xpos, ypos, width_mm, height_mm);

      // Print Vidmodes
      let mut count_modes: c_int = 0;
      let raw_modes = glfw::GetVideoModes(*m, &mut count_modes);
      let modes = std::slice::from_raw_parts(raw_modes, count_modes as usize);
      for v in modes.iter() {
        println!("\t{}x{}@{}hz", v.width, v.height, v.refreshRate);
      }
    }

    glfw::WindowHint(glfw::RESIZABLE, glfw::FALSE);
    let window = glfw::CreateWindow(640, 480, CString::new("test title").unwrap().as_ptr(), std::ptr::null_mut(), std::ptr::null_mut());
    glfw::SetWindowPosCallback(window, window_pos_callback);
    glfw::SetKeyCallback(window, key_callback);
    glfw::MakeContextCurrent(window);

    gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

    let cursor = glfw::CreateStandardCursor(glfw::CROSSHAIR_CURSOR);
    glfw::SetCursor(window, cursor);

    gl::ClearColor(0.5, 0.5, 1.0, 1.0);
    while glfw::WindowShouldClose(window) == glfw::FALSE {
      gl::Clear(gl::COLOR_BUFFER_BIT);

      glfw::PollEvents();
      glfw::SwapBuffers(window);
    }

    glfw::DestroyWindow(window);
    glfw::Terminate();
  }
}
