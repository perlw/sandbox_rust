extern crate glfw;

fn main() {
  glfw::init();

  println!("{}", glfw::get_version_string());

  let window = glfw::create_window(640, 480, "test title").unwrap();
  while !window.should_close() {
    glfw::poll_events();
    window.swap_buffers();
  }
}
