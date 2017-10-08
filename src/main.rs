extern crate glfw_sys as glfw;

use std::fs::File;
use std::io::prelude::*;

mod bedrock;

struct TestSystem {
    dummy: i32,
    foobar: Foobar,
    baz: String,
}

impl bedrock::kronos::System<FooTypes> for TestSystem {
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
    let mut window = picasso
        .new_window()
        .opengl_context_version(3, 3)
        .opengl_context_debug(true)
        .resizable(false)
        .with_context_config(|config| {
            config.clear_color(0.5, 0.5, 1.0, 1.0).debug(|msg| {
                println!("GL ERR: {}", msg)
            });
        })
        .create()
        .unwrap();

    let mut window2 = picasso
        .new_window()
        .opengl_context_version(3, 3)
        .opengl_context_debug(true)
        .resizable(false)
        .with_context_config(|config| {
            config.clear_color(1.0, 0.0, 1.0, 1.0).debug(|msg| {
                println!("GL ERR2: {}", msg)
            });
        })
        .create()
        .unwrap();

    let shader_handle = window
        .with_context(|context| {
            let mut vert_source: Vec<u8> = Vec::new();
            let mut frag_source: Vec<u8> = Vec::new();
            File::open("assets/shaders/simple.vert")
                .and_then(|mut file| file.read_to_end(&mut vert_source))
                .unwrap();
            File::open("assets/shaders/simple.frag")
                .and_then(|mut file| file.read_to_end(&mut frag_source))
                .unwrap();

            context.new_shader(&vert_source, &frag_source)
        })
        .unwrap();

    let square_handle = window
        .with_context(|context| {
            let handle = context.new_buffergroup();

            context.with_buffergroup(handle, |group| {
                let buf = group.new_buffer();
                // buf.set_data(?)
            });

            handle
        });

    let shader_handle2 = window2
        .with_context(|context| {
            let mut vert_source: Vec<u8> = Vec::new();
            let mut frag_source: Vec<u8> = Vec::new();
            File::open("assets/shaders/simple.vert")
                .and_then(|mut file| file.read_to_end(&mut vert_source))
                .unwrap();
            File::open("assets/shaders/simple_red.frag")
                .and_then(|mut file| file.read_to_end(&mut frag_source))
                .unwrap();

            context.new_shader(&vert_source, &frag_source)
        })
        .unwrap();

    println!("Shaders {} {}", shader_handle, shader_handle2);

    window.keyboard_callback(|window, key, scancode| {
        println!("KEY: {}", key);
        if key == 256 {
            window.set_should_close(true);
        }
    });

    let mut last_tick = unsafe { glfw::GetTime() as f64 };
    while !window.should_close() && !window2.should_close() {
        let tick = unsafe { glfw::GetTime() as f64 };
        let delta = tick - last_tick;
        last_tick = tick;

        kronos.update(delta);

        window.with_context(|context| {
            context.clear();

            context.with_shader_and_buffergroup(shader_handle, square_handle, |shader, square| {
                shader.activate();
                //square.render(?);
            });
        });
        window.swap_buffers();

        window2.with_context(|context| context.clear());
        window2.swap_buffers();

        picasso.poll_events();
    }
}
