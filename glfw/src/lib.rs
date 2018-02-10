extern crate libc;

use libc::{c_char, c_double, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

mod sys;

// TODO: Can we do anything more robust than print to stderr?
extern fn rust_glfw_error_callback(error: c_int, description: *const c_char) {
    let desc = unsafe { CStr::from_ptr(description) };
    eprintln!("GLFW Error {}: {}", error, desc.to_string_lossy().into_owned());
}

extern fn rust_glfw_cursor_pos_callback(window: *mut sys::GLFWwindow,
                                        xpos: c_double,
                                        ypos: c_double) {
    let user_ptr = unsafe { sys::glfwGetWindowUserPointer(window) };
    let cb: &mut Callbacks = unsafe { &mut *(user_ptr as *mut Callbacks) };
    if let Some(ref mut callback) = cb.cursor_pos_cb {
        callback(xpos as f64, ypos as f64);
    }
}

struct Callbacks {
    cursor_pos_cb: Option<Box<FnMut(f64, f64)>>,
}

impl Callbacks {
    fn new() -> Callbacks {
        Callbacks {
            cursor_pos_cb: None,
        }
    }
}

pub struct Glfw {}

pub struct Window {
    window: *mut sys::GLFWwindow,
    callbacks: Box<Callbacks>,
}

pub fn init() -> Glfw {
    unsafe {
        sys::glfwSetErrorCallback(rust_glfw_error_callback);
        sys::glfwInit()
    };
    Glfw {}
}

pub fn get_version() -> (i32, i32, i32) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut rev: c_int = 0;

    unsafe {
        sys::glfwGetVersion(&mut major, &mut minor, &mut rev);
    }

    (major as i32, minor as i32, rev as i32)
}

pub fn get_version_string() -> String {
    unsafe {
        CStr::from_ptr(sys::glfwGetVersionString())
            .to_string_lossy()
            .into_owned()
    }
}

impl Drop for Glfw {
    fn drop(&mut self) {
        unsafe { sys::glfwTerminate(); }
    }
}

impl Glfw {
    // TODO: Add monitor/share arguments.
    pub fn create_window(&mut self,
                         width: i32,
                         height: i32,
                         title: &str) -> Window {
        let title = CString::new(title).unwrap();
        let glfw_window = unsafe { sys::glfwCreateWindow(width as c_int,
                                                         height as c_int,
                                                         title.as_ptr(),
                                                         ptr::null(),
                                                         ptr::null()) };
        Window::new(glfw_window)
    }

    pub fn poll_events(&mut self) {
        unsafe { sys::glfwPollEvents(); }
    }

    pub fn wait_events(&mut self) {
        unsafe { sys::glfwWaitEvents(); }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { sys::glfwDestroyWindow(self.window); }
    }
}

impl Window {
    fn new(window: *mut sys::GLFWwindow) -> Window {
        let mut cb = Box::new(Callbacks::new());

        unsafe {
            // Associate this window's Rust callback table with the window in GLFW.
            let user_ptr = &mut *cb as *mut _ as *mut c_void;
            sys::glfwSetWindowUserPointer(window, user_ptr);

            // Set up trampoline callbacks.
            sys::glfwSetCursorPosCallback(window, rust_glfw_cursor_pos_callback);
        }

        Window {
            window: window,
            callbacks: cb,
        }
    }

    pub fn set_cursor_pos_callback(&mut self, callback: Box<FnMut(f64, f64)>) {
        self.callbacks.cursor_pos_cb = Some(callback);
    }

    pub fn make_context_current(&mut self) {
        unsafe { sys::glfwMakeContextCurrent(self.window); }
    }

    pub fn window_should_close(&mut self) -> bool {
        unsafe { sys::glfwWindowShouldClose(self.window) == sys::GLFW_TRUE }
    }

    pub fn swap_buffers(&mut self) {
        unsafe { sys::glfwSwapBuffers(self.window); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version_test() {
        let (major, minor, rev) = get_version();
        assert_eq!(3, major);
        assert_eq!(2, minor);
        assert_eq!(1, rev);
    }

    #[test]
    fn get_version_string_test() {
        let version = get_version_string();
        assert!(version.contains("3.2.1"));
    }
}
