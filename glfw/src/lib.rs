extern crate libc;

use libc::c_int;
use std::ffi;

mod sys;

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
        ffi::CStr::from_ptr(sys::glfwGetVersionString())
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
