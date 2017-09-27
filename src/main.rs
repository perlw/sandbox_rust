extern crate libc;
use std::ffi::{CStr, CString};

extern crate glfw_sys as glfw;
#[allow(unused)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod bedrock;

struct TestSystem {
    dummy: i32,
    foobar: Foobar,
    baz: String,
}

impl bedrock::kronos::HasSystem<FooTypes> for TestSystem {
    fn start(&mut self) -> bool {
        self.dummy += 1;
        println!(
            "dummy {}, foobar {}, baz {}",
            self.dummy,
            self.foobar.dummy,
            self.baz
        );
        true
    }

    fn stop(&mut self) -> bool {
        true
    }

    fn update(&mut self, delta: f64) {
        println!("UPDATE SYSTEM {}", unsafe { glfw::GetTime() as f64 });
    }

    fn message(&mut self, msg: &FooTypes) {
        println!("MESSAGE SYSTEM");
        match msg {
            &FooTypes::Num(x) => println!("\tNum => {}", x),
            &FooTypes::Foobar(ref x) => println!("\tFoobar => {}", x.dummy),
        }
    }
}

extern "system" fn debug_callback(source: gl::types::GLenum, gltype: gl::types::GLenum, id: gl::types::GLuint, severity: gl::types::GLenum, length: gl::types::GLsizei, message: *const gl::types::GLchar, userParam: *mut std::os::raw::c_void) {
    unsafe {
        println!("GL ERR: {}", CStr::from_ptr(message).to_string_lossy());
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
        AnotherFoo { dummy: foo.dummy }
    }
}

enum FooTypes {
    Num(i32),
    Foobar(Foobar),
}

fn print_map_val(types: &FooTypes) {
    match types {
        &FooTypes::Num(x) => println!("i32 => {}", x),
        &FooTypes::Foobar(ref x) => println!("Foobar => {}", x.dummy),
    }
}

use std::collections::HashMap;
fn main() {
    let mut kronos: bedrock::Kronos<FooTypes> = bedrock::Kronos::new();
    kronos.register(
        "test_system",
        false,
        1.0,
        TestSystem {
            dummy: 0,
            foobar: Foobar { dummy: 42 },
            baz: String::from("what is the meaning of life"),
        },
    );
    kronos.register(
        "test_system2",
        false,
        1.0,
        TestSystem {
            dummy: 42,
            foobar: Foobar { dummy: 1337 },
            baz: String::from("yes indeed"),
        },
    );
    kronos.start_system("test_system");
    kronos.start_system("test_system2");

    let foo = Foobar { dummy: 1 };
    let a_foo = AnotherFoo::from(foo);
    println!("FOO {}", a_foo.dummy);

    let mut map = HashMap::<&str, FooTypes>::new();
    map.insert("int", FooTypes::Num(1337));
    map.insert("foo", FooTypes::Foobar(Foobar { dummy: 42 }));
    let map_int = map.get("int").unwrap();
    let map_foo = map.get("foo").unwrap();
    print_map_val(map_int);
    print_map_val(map_foo);

    kronos.post_message("test_system", &FooTypes::Num(1337));
    kronos.emit_message(&FooTypes::Num(42));

    let picasso = bedrock::Picasso::new();
    let window = picasso
        .new_window()
        .opengl_context_version(4, 3)
        .opengl_context_debug(true)
        .resizable(false)
        .create()
        .unwrap();
    window.make_context_current();

    unsafe {
        gl::load_with(|s| glfw::GetProcAddress(CString::new(s).unwrap().as_ptr()));

        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(debug_callback, std::ptr::null());

        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearDepth(1.0);
        gl::DepthFunc(gl::LESS);
        gl::Viewport(0, 0, 640, 480);
        gl::ClearColor(0.5, 0.5, 1.0, 1.0);
    }
    let mut last_tick = unsafe { glfw::GetTime() as f64 };
    while !window.should_close() {
        let tick = unsafe { glfw::GetTime() as f64 };
        let delta = tick - last_tick;
        last_tick = tick;

        kronos.update(delta);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Render stuff
        window.swap_buffers();

        unsafe {
            glfw::PollEvents();
        }
    }
}
