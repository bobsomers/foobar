use std::os::raw::{c_char, c_double, c_int, c_void};

pub const TRUE: c_int = 1;
pub const FALSE: c_int = 0;

pub const CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub const CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub const OPENGL_FORWARD_COMPAT: c_int = 0x00022006;
pub const OPENGL_PROFILE: c_int = 0x00022008;

pub const OPENGL_ANY_PROFILE: c_int = 0;
pub const OPENGL_CORE_PROFILE: c_int = 0x00032001;
pub const OPENGL_COMPAT_PROFILE: c_int = 0x00032002;

pub type GLFWglproc = *const c_void;
pub type GLFWerrorfun = extern "C" fn(c_int, *const c_char);

pub enum GLFWmonitor {}
pub enum GLFWwindow {}

extern {
    pub fn glfwSetErrorCallback(cbfun: GLFWerrorfun) -> GLFWerrorfun;

    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();

    pub fn glfwGetProcAddress(procname: *const c_char) -> GLFWglproc;

    pub fn glfwGetVersion(major: *mut c_int,
                          minor: *mut c_int,
                          rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;

    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwDefaultWindowHints();

    pub fn glfwCreateWindow(width: c_int,
                            height: c_int,
                            title: *const c_char,
                            monitor: *const GLFWmonitor,
                            share: *const GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);

    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut c_void;
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *mut c_void);
    pub fn glfwGetFramebufferSize(window: *mut GLFWwindow,
                                  width: *mut c_int,
                                  height: *mut c_int);

    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);

    pub fn glfwPollEvents();
    pub fn glfwWaitEvents();

    pub fn glfwSetWindowPosCallback(window: *mut GLFWwindow,
                                    cbfun: extern fn(*mut GLFWwindow, c_int, c_int));
    pub fn glfwSetWindowSizeCallback(window: *mut GLFWwindow,
                                     cbfun: extern fn(*mut GLFWwindow, c_int, c_int));
    pub fn glfwSetWindowCloseCallback(window: *mut GLFWwindow,
                                      cbfun: extern fn(*mut GLFWwindow));
    pub fn glfwSetWindowRefreshCallback(window: *mut GLFWwindow,
                                        cbfun: extern fn(*mut GLFWwindow));
    pub fn glfwSetWindowFocusCallback(window: *mut GLFWwindow,
                                      cbfun: extern fn(*mut GLFWwindow, c_int));
    pub fn glfwSetWindowIconifyCallback(window: *mut GLFWwindow,
                                        cbfun: extern fn(*mut GLFWwindow, c_int));
    pub fn glfwSetFramebufferSizeCallback(window: *mut GLFWwindow,
                                          cbfun: extern fn(*mut GLFWwindow, c_int, c_int));
    pub fn glfwSetCursorPosCallback(window: *mut GLFWwindow,
                                    cbfun: extern fn(*mut GLFWwindow, c_double, c_double));
}
