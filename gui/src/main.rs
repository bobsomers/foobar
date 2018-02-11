extern crate glfw;
extern crate gl;

fn main() {
    let mut glfw = glfw::init();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::ForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::Profile::Core));

    let mut window = glfw.create_window(800, 600, "Hello, GLFW!");

    window.make_context_current();

    gl::load_with(|s| window.get_proc_address(s));

    while !window.window_should_close() {
        glfw.wait_events();

        for event in window.events.drain(..) {
            println!("{:?}", event);
        }

        let (fb_width, fb_height) = window.get_framebuffer_size();

        unsafe {
            gl::Viewport(0, 0, fb_width, fb_height);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}
