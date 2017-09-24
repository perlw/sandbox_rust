extern crate glfw_sys as glfw;
mod gl {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CString;

fn main() {
  unsafe {
    glfw::Init();
    let window = glfw::CreateWindow(640, 480, CString::new("test title").unwrap().as_ptr(), std::ptr::null_mut(), std::ptr::null_mut());
    glfw::MakeContextCurrent(window);
    gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

    gl::ClearColor(0.5, 0.5, 1.0, 1.0);
    while glfw::WindowShouldClose(window) == 0 {
      gl::Clear(gl::COLOR_BUFFER_BIT);

      glfw::PollEvents();
      glfw::SwapBuffers(window);
    }

    glfw::DestroyWindow(window);
    glfw::Terminate();
  }
}
