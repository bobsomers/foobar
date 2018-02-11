use libc::{c_char, c_double, c_int};
use std::collections::VecDeque;
use std::ffi::CStr;

use super::Event;
use super::sys;

pub fn register_error_callback() {
    unsafe {
        sys::glfwSetErrorCallback(rust_glfw_error);
    }
}

pub fn register_window_callbacks(window: *mut sys::GLFWwindow) {
    unsafe {
        sys::glfwSetWindowPosCallback(window, rust_glfw_window_pos);
        sys::glfwSetWindowSizeCallback(window, rust_glfw_window_size);
        sys::glfwSetWindowCloseCallback(window, rust_glfw_window_close);
        sys::glfwSetWindowRefreshCallback(window, rust_glfw_window_refresh);
        sys::glfwSetWindowFocusCallback(window, rust_glfw_window_focus);
        sys::glfwSetWindowIconifyCallback(window, rust_glfw_window_iconify);
        sys::glfwSetFramebufferSizeCallback(window, rust_glfw_framebuffer_size);
        sys::glfwSetCursorPosCallback(window, rust_glfw_cursor_pos);
        // TODO: moar
    }
}

struct EventQueue<'a> {
    events: &'a mut VecDeque<Event>,
}

impl<'a> EventQueue<'a> {
    fn from_window(window: *mut sys::GLFWwindow) -> EventQueue<'a> {
        let user_ptr = unsafe { sys::glfwGetWindowUserPointer(window) };
        EventQueue {
            events: unsafe { &mut *(user_ptr as *mut VecDeque<Event>) },
        }
    }

    fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }
}

// TODO: Can we do anything more robust than print to stderr?
extern fn rust_glfw_error(error: c_int, description: *const c_char) {
    let desc = unsafe { CStr::from_ptr(description) };
    eprintln!("GLFW Error {}: {}", error, desc.to_string_lossy().into_owned());
}

extern fn rust_glfw_window_pos(window: *mut sys::GLFWwindow,
                               xpos: c_int,
                               ypos: c_int) {
    EventQueue::from_window(window).push(Event::WindowPos {
        x: xpos as i32,
        y: ypos as i32,
    });
}

extern fn rust_glfw_window_size(window: *mut sys::GLFWwindow,
                                width: i32,
                                height: i32) {
    EventQueue::from_window(window).push(Event::WindowSize {
        width: width as i32,
        height: height as i32,
    });
}

extern fn rust_glfw_window_close(window: *mut sys::GLFWwindow) {
    EventQueue::from_window(window).push(Event::WindowClose);
}

extern fn rust_glfw_window_refresh(window: *mut sys::GLFWwindow) {
    EventQueue::from_window(window).push(Event::WindowRefresh);
}

extern fn rust_glfw_window_focus(window: *mut sys::GLFWwindow,
                                 focused: c_int) {
    EventQueue::from_window(window).push(Event::WindowFocus(
        focused == sys::GLFW_TRUE
    ));
}

extern fn rust_glfw_window_iconify(window: *mut sys::GLFWwindow,
                                   iconified: c_int) {
    EventQueue::from_window(window).push(Event::WindowIconify(
        iconified == sys::GLFW_TRUE
    ));
}

extern fn rust_glfw_framebuffer_size(window: *mut sys::GLFWwindow,
                                     width: c_int,
                                     height: c_int) {
    EventQueue::from_window(window).push(Event::FramebufferSize {
        width: width as i32,
        height: height as i32,
    });
}

extern fn rust_glfw_cursor_pos(window: *mut sys::GLFWwindow,
                               xpos: c_double,
                               ypos: c_double) {
    EventQueue::from_window(window).push(Event::CursorPos {
        x: xpos as f64,
        y: ypos as f64,
    });
}
