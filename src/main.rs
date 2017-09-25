extern crate glfw_sys as glfw;
mod gl {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod bedrock;

extern crate libc;

use std::ffi::{CStr, CString};
use libc::{c_char, c_int};

#[allow(unused)]
extern fn error_callback(error: c_int, description: *const c_char) {
  unsafe {
    println!("ERROR {}", CStr::from_ptr(description).to_str().unwrap());
    if error == glfw::NOT_INITIALIZED {
      println!("YAS");
    }
  }
}

#[allow(unused)]
extern fn window_pos_callback(window: *mut glfw::Window, xpos: c_int, ypos: c_int) {
  println!("WindowPos: {}x{}", xpos, ypos);
}

#[allow(unused)]
extern fn key_callback(window: *mut glfw::Window, key: c_int, scancode: c_int, action: c_int, mods: c_int) {
  if key == glfw::KEY_ESCAPE {
    unsafe {
      glfw::SetWindowShouldClose(window, glfw::TRUE);
    }
  }
}

struct TestSystem {
  dummy: i32,
}

impl bedrock::kronos::HasSystem for TestSystem {
  fn start(&mut self) -> bool {
    println!("START SYSTEM");
    self.dummy += 1;
    true
  }

  fn stop(&mut self) -> bool {
    println!("STOP SYSTEM");
    true
  }

  fn update(&mut self, delta: f64) {
    println!("UPDATE SYSTEM {}", unsafe { glfw::GetTime() as f64 });
  }

  fn message(&mut self) {
    println!("MESSAGE SYSTEM");
  }
}

struct Foobar {
  dummy: i32,
}

struct AnotherFoo {
  dummy: i32,
}

impl From<Foobar> for AnotherFoo {
  fn from(foo: Foobar) -> Self {
    AnotherFoo{ dummy: foo.dummy }
  }
}

enum FooTypes {
  i32(i32),
  Foobar(Foobar),
}

fn print_map_val(types: &FooTypes) {
  match types {
    &FooTypes::i32(x) => println!("i32 => {}", x),
    &FooTypes::Foobar(ref x) => println!("Foobar => {}", x.dummy)
  }
}

use std::collections::HashMap;
fn main() {
  let mut kronos = bedrock::Kronos::new();
  kronos.register("test_system", false, 1.0, TestSystem{
    dummy: 0,
  });
  kronos.start_system("test_system");

  let foo = Foobar{ dummy: 1 };
  let a_foo = AnotherFoo::from(foo);
  println!("FOO {}", a_foo.dummy);

  let mut map = HashMap::<&str, FooTypes>::new();
  map.insert("int", FooTypes::i32(1337));
  map.insert("foo", FooTypes::Foobar(Foobar{ dummy: 42 }));
  let map_int = map.get("int").unwrap();
  let map_foo = map.get("foo").unwrap();
  print_map_val(map_int);
  print_map_val(map_foo);

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

    let mut last_tick = glfw::GetTime() as f64;
    while glfw::WindowShouldClose(window) == glfw::FALSE {
      let tick = glfw::GetTime() as f64;
      let delta = tick - last_tick;
      last_tick = tick;

      kronos.update(delta);

      gl::Clear(gl::COLOR_BUFFER_BIT);
      // Render stuff
      glfw::SwapBuffers(window);

      glfw::PollEvents();
    }

    glfw::DestroyWindow(window);
    glfw::Terminate();
  }
}
