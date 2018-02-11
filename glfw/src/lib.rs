use std::collections::VecDeque;
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_void};
use std::ptr;

mod cb;
mod sys;

#[derive(Debug)]
pub enum Event {
    WindowPos { x: i32, y: i32 },
    WindowSize { width: i32, height: i32 },
    WindowClose,
    WindowRefresh,
    WindowFocus(bool),
    WindowIconify(bool),
    FramebufferSize { width: i32, height: i32 },
    CursorPos { x: f64, y: f64 },
}

#[derive(Debug)]
pub enum Profile {
    Any,
    Compat,
    Core,
}

#[derive(Debug)]
pub enum WindowHint {
    ContextVersion(i32, i32),
    ForwardCompat(bool),
    OpenGlProfile(Profile),
}

pub struct Glfw {}

pub struct Window {
    window: *mut sys::GLFWwindow,
    pub events: Box<VecDeque<Event>>,
}

pub fn init() -> Glfw {
    cb::register_error_callback();
    unsafe { sys::glfwInit(); }
    Glfw {}
}

pub fn get_version() -> (i32, i32, i32) {
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut rev: c_int = 0;
    unsafe { sys::glfwGetVersion(&mut major, &mut minor, &mut rev); }
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

fn set_hint(hint: c_int, value: c_int) {
    unsafe { sys::glfwWindowHint(hint, value); }
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

    pub fn window_hint(&mut self, hint: WindowHint) {
        match hint {
            WindowHint::ContextVersion(major, minor) => {
                set_hint(sys::CONTEXT_VERSION_MAJOR, major);
                set_hint(sys::CONTEXT_VERSION_MINOR, minor);
            },
            WindowHint::ForwardCompat(compat) => {
                if compat {
                    set_hint(sys::OPENGL_FORWARD_COMPAT, sys::TRUE);
                } else {
                    set_hint(sys::OPENGL_FORWARD_COMPAT, sys::FALSE);
                }
            },
            WindowHint::OpenGlProfile(profile) => {
                match profile {
                    Profile::Any => set_hint(sys::OPENGL_PROFILE, sys::OPENGL_ANY_PROFILE),
                    Profile::Core => set_hint(sys::OPENGL_PROFILE, sys::OPENGL_CORE_PROFILE),
                    Profile::Compat => set_hint(sys::OPENGL_PROFILE, sys::OPENGL_COMPAT_PROFILE),
                }
            }
        }
    }

    pub fn default_window_hints(&mut self) {
        unsafe { sys::glfwDefaultWindowHints(); }
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
        }

        // Set up event injection callbacks.
        cb::register_window_callbacks(window);

        Window {
            window: window,
            events: events,
        }
    }

    pub fn get_proc_address(&mut self, symbol: &str) -> sys::GLFWglproc {
        let symbol = CString::new(symbol).unwrap();
        unsafe { sys::glfwGetProcAddress(symbol.as_ptr()) }
    }

    pub fn make_context_current(&mut self) {
        unsafe { sys::glfwMakeContextCurrent(self.window); }
    }

    pub fn get_framebuffer_size(&mut self) -> (i32, i32) {
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        unsafe { sys::glfwGetFramebufferSize(self.window, &mut width, &mut height); }
        (width as i32, height as i32)
    }

    pub fn window_should_close(&mut self) -> bool {
        unsafe { sys::glfwWindowShouldClose(self.window) == sys::TRUE }
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
