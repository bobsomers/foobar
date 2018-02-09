#[macro_use]
extern crate lazy_static;
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::CStr;
use std::sync::Mutex;

mod sys;

#[derive(Clone, Debug)]
pub enum Error {
    Unknown(String),
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

    fn unknown() -> Error {
        Error::Unknown(String::from("No error was set by the GLFW error callback."))
    }
}

lazy_static! {
    static ref RUST_GLFW_ERROR: Mutex<Option<Error>> = Mutex::new(None);
}

extern fn rust_glfw_error_callback(error: c_int, description: *const c_char) {
    let description = unsafe { CStr::from_ptr(description) };
    *(RUST_GLFW_ERROR.lock().unwrap()) = Some(Error::from_glfw(error, description))
}

fn get_error_and_clear() -> Error {
    let mut result = RUST_GLFW_ERROR.lock().unwrap();
    let err = result.clone();
    *result = None;
    match err {
        Some(e) => e,
        None => Error::unknown(),
    }
}

pub struct Glfw {}

pub fn init() -> Result<Glfw, Error> {
    let result = unsafe {
        sys::glfwSetErrorCallback(rust_glfw_error_callback);
        sys::glfwInit()
    };
    if result == sys::GLFW_TRUE {
        Ok(Glfw {})
    } else {
        Err(get_error_and_clear())
    }
}

impl Drop for Glfw {
    fn drop(&mut self) {
        unsafe { sys::glfwTerminate(); }
    }
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
