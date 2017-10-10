extern crate glfw_sys as glfw;
extern crate stb_image_sys as stbi;
extern crate cgmath;

use std::fs::File;
use std::io::prelude::*;
use cgmath::prelude::*;

mod bedrock;
use bedrock::picasso::buffer::{BufferHandle, BufferTarget, BufferType};
use bedrock::picasso::shader::ShaderUniformData;

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
        .title("blue")
        .opengl_context_version(3, 3)
        .opengl_context_debug(true)
        .resizable(false)
        .with_context_config(|config| {
            config.clear_color(0.5, 0.5, 1.0, 1.0).debug(|msg| {
                println!("BLUE GL INFO: {}", msg)
            });
        })
        .create()
        .unwrap();

    let mut window2 = picasso
        .new_window()
        .title("purple")
        .opengl_context_version(3, 3)
        .opengl_context_debug(true)
        .resizable(false)
        .with_context_config(|config| {
            config.clear_color(1.0, 0.0, 1.0, 1.0).debug(|msg| {
                println!("PURPLE GL INFO: {}", msg)
            });
        })
        .create()
        .unwrap();

    let mut texture_handle = 0 as u32;
    window.with_context(|context| {
        let mut font_buffer: Vec<u8> = Vec::new();
        File::open("assets/fonts/cp437_8x8.png")
            .and_then(|mut file| file.read_to_end(&mut font_buffer))
            .unwrap();

        let mut w = 0;
        let mut h = 0;
        let raw_image = unsafe {
            stbi::load_from_memory(
                font_buffer.as_mut_ptr() as *mut _,
                font_buffer.len() as i32,
                &mut w,
                &mut h,
                std::ptr::null_mut(),
                3,
            )
        };
        println!("IMAGE SIZE {} {}", w, h);

        use bedrock::picasso::context::gl;
        unsafe {
            gl::GenTextures(1, &mut texture_handle);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_handle);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                w,
                h,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                raw_image as *mut _,
            );
        }

        unsafe {
            stbi::free(raw_image);
        }
    });

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

            let handle = context.new_shader(&vert_source, &frag_source);

            context.with_shader(handle.unwrap(), |shader| {
                let ortho = cgmath::ortho::<f32>(0., 640., 0., 480., 0., 1.);
                let model = cgmath::Matrix4::<f32>::identity();

                let pmatrix_uniform = shader.get_uniform_location("pMatrix");
                shader.set_uniform(pmatrix_uniform, ShaderUniformData::Mat4(ortho));
                let mvmatrix_uniform = shader.get_uniform_location("mvMatrix");
                shader.set_uniform(mvmatrix_uniform, ShaderUniformData::Mat4(model));

                let tex_uniform = shader.get_uniform_location("tex");
                shader.set_uniform(tex_uniform, ShaderUniformData::Int(0));
            });

            handle
        })
        .unwrap();

    let square_handle = window.with_context(|context| {
        let handle = context.new_buffergroup();

        let mut vert_buf = 0 as BufferHandle;
        let mut coord_buf = 0 as BufferHandle;
        context.with_buffergroup(handle, |group| {
            let size = 320;
            vert_buf = group.new_buffer(BufferTarget::ArrayBuffer);
            group.with_buffer(vert_buf, |buffer| {
                let data: Vec<i32> = vec![0, 0, size, size, 0, size, 0, 0, size, 0, size, size];
                buffer.set_data(data)
            });

            coord_buf = group.new_buffer(BufferTarget::ArrayBuffer);
            group.with_buffer(coord_buf, |buffer| {
                let data: Vec<f32> = vec![0., 1., 1., 0., 0., 0., 0., 1., 1., 1., 1., 0.];
                buffer.set_data(data)
            });
        });

        context.with_shader_and_buffergroup(shader_handle, handle, |shader, group| {
            let vertex_loc = shader.get_attrib_location("vertex");
            group.with_buffer(vert_buf, |buffer| {
                buffer.vertex_attrib(vertex_loc as u32, 2, BufferType::Int)
            });

            let coord_loc = shader.get_attrib_location("coord");
            group.with_buffer(coord_buf, |buffer| {
                buffer.vertex_attrib(coord_loc as u32, 2, BufferType::Float)
            });
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

    window.keyboard_callback(|window, key, scancode| {
        println!("KEY: {}", key);
        if key == 256 {
            window.set_should_close(true);
        }
    });

    let mut fps_second = 0.0 as f64;
    let mut fps_count = 0;
    let mut last_tick = unsafe { glfw::GetTime() as f64 };
    while !window.should_close() && !window2.should_close() {
        let tick = unsafe { glfw::GetTime() as f64 };
        let delta = tick - last_tick;
        last_tick = tick;

        fps_count += 1;
        fps_second += delta;
        if fps_second >= 1.0 {
            println!("FPS: {}", fps_count);
            fps_second = 0.0;
            fps_count = 0;
        }

        kronos.update(delta);

        window.with_context(|context| {
            context.clear();

            // Thoughts: Better way to deal with requesting interfaces, or let other
            // system/module/etc handle abstraction?
            context.with_shader_and_buffergroup(shader_handle, square_handle, |shader, square| {
                use bedrock::picasso::context::gl;
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture_handle);
                }

                square.draw();
            });
        });
        window.swap_buffers();

        window2.with_context(|context| { context.clear(); });
        window2.swap_buffers();

        picasso.poll_events();
    }
}
