extern crate glfw;

fn main() {
    let mut glfw = glfw::init();
    let mut window = glfw.create_window(800, 600, "Hello, GLFW!");
    window.make_context_current();
    while !window.window_should_close() {
        glfw.wait_events();

        println!("---- FRAME ---------------------------");
        for event in window.events.drain(..) {
            println!("{:?}", event);
        }

        window.swap_buffers();
    }
}
