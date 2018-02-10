extern crate libc;

use libc::{c_int, c_void};
use std::collections::VecDeque;
use std::ffi::{CStr, CString};
use std::ptr;

mod cb;
mod sys;

#[derive(Debug)]
pub enum Event {
    CursorPos(f64, f64),
}

pub struct Glfw {}

pub struct Window {
    window: *mut sys::GLFWwindow,
    pub events: Box<VecDeque<Event>>,
}

pub fn init() -> Glfw {
    unsafe {
        sys::glfwSetErrorCallback(cb::rust_glfw_error);
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
        let mut events = Box::new(VecDeque::new());

        unsafe {
            // Associate this window's event queue with the window in GLFW.
            let user_ptr = &mut *events as *mut _ as *mut c_void;
            sys::glfwSetWindowUserPointer(window, user_ptr);

            // Set up event injection callbacks.
            sys::glfwSetCursorPosCallback(window, cb::rust_glfw_cursor_pos);

            // TODO: moar
        }

        Window {
            window: window,
            events: events,
        }
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
