extern crate glfw;

fn main() {
    let mut glfw = glfw::init().unwrap();
    let mut window = glfw.create_window(800, 600, "Hello, GLFW!").unwrap();
    window.make_context_current();
    while !window.window_should_close() {
        glfw.poll_events();
        window.swap_buffers();
    }
}
