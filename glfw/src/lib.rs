#[macro_use]
extern crate lazy_static;
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;

mod sys;

#[derive(Clone, Debug)]
pub enum Error {
    NotInitialized(String),
    NoCurrentContext(String),
    InvalidEnum(String),
    InvalidValue(String),
    OutOfMemory(String),
    ApiUnavailable(String),
    VersionUnavailable(String),
    PlatformError(String),
    FormatUnavailable(String),
    NoWindowContext(String),
}

impl Error {
    fn from_glfw(code: c_int, description: &CStr) -> Error {
        let description = description.to_string_lossy().into_owned();
        match code {
            sys::GLFW_NOT_INITIALIZED => Error::NotInitialized(description),
            sys::GLFW_NO_CURRENT_CONTEXT => Error::NoCurrentContext(description),
            sys::GLFW_INVALID_ENUM => Error::InvalidEnum(description),
            sys::GLFW_INVALID_VALUE => Error::InvalidValue(description),
            sys::GLFW_OUT_OF_MEMORY => Error::OutOfMemory(description),
            sys::GLFW_API_UNAVAILABLE => Error::ApiUnavailable(description),
            sys::GLFW_VERSION_UNAVAILABLE => Error::VersionUnavailable(description),
            sys::GLFW_PLATFORM_ERROR => Error::PlatformError(description),
            sys::GLFW_FORMAT_UNAVAILABLE => Error::FormatUnavailable(description),
            sys::GLFW_NO_WINDOW_CONTEXT => Error::NoWindowContext(description),
            _ => panic!("GLFW returned unknown error code: {}: {}", code, description)
        }
    }
}

lazy_static! {
    static ref RUST_GLFW_ERROR: Mutex<Option<Error>> = Mutex::new(None);
}

// TODO: After initialization, should this just panic! with an error message?
// It's likely too slow to call after every function.
extern fn rust_glfw_error_callback(error: c_int, description: *const c_char) {
    let description = unsafe { CStr::from_ptr(description) };
    *(RUST_GLFW_ERROR.lock().unwrap()) = Some(Error::from_glfw(error, description))
}

fn check_for_error() -> Result<(), Error> {
    let mut result = RUST_GLFW_ERROR.lock().unwrap();
    let err = result.clone();
    *result = None;
    match err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

pub struct Glfw {}

pub struct Window {
    window: *mut sys::GLFWwindow,
}

pub fn init() -> Result<Glfw, Error> {
    unsafe {
        sys::glfwSetErrorCallback(rust_glfw_error_callback);
        sys::glfwInit()
    };
    check_for_error()?;
    Ok(Glfw {})
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
                         title: &str) -> Result<Window, Error> {
        let title = CString::new(title).unwrap();
        let glfw_window = unsafe { sys::glfwCreateWindow(width as c_int,
                                                         height as c_int,
                                                         title.as_ptr(),
                                                         ptr::null(),
                                                         ptr::null()) };
        check_for_error()?;
        Ok(Window { window: glfw_window })
    }

    pub fn poll_events(&mut self) {
        unsafe { sys::glfwPollEvents(); }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { sys::glfwDestroyWindow(self.window); }
    }
}

impl Window {
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
