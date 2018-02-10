use libc::{c_char, c_double, c_int};
use std::collections::VecDeque;
use std::ffi::CStr;

use super::Event;
use super::sys;

// TODO: Can we do anything more robust than print to stderr?
pub extern fn rust_glfw_error(error: c_int, description: *const c_char) {
    let desc = unsafe { CStr::from_ptr(description) };
    eprintln!("GLFW Error {}: {}", error, desc.to_string_lossy().into_owned());
}

pub extern fn rust_glfw_cursor_pos(window: *mut sys::GLFWwindow,
                                   xpos: c_double,
                                   ypos: c_double) {
    let user_ptr = unsafe { sys::glfwGetWindowUserPointer(window) };
    let events: &mut VecDeque<Event> = unsafe { &mut *(user_ptr as *mut VecDeque<Event>) };
    events.push_back(Event::CursorPos(xpos as f64, ypos as f64));
}
