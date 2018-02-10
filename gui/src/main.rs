extern crate glfw;

fn main() {
    let mut glfw = glfw::init();
    let mut window = glfw.create_window(800, 600, "Hello, GLFW!");
    window.set_cursor_pos_callback(Box::new(|xpos: f64, ypos: f64| {
        println!("xpos: {}, ypos: {}", xpos, ypos);
    }));
    window.make_context_current();
    while !window.window_should_close() {
        glfw.wait_events();
        window.swap_buffers();
    }
}
