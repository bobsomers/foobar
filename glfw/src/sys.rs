use libc::{c_char, c_double, c_int, c_void};

pub const GLFW_TRUE: c_int = 1;

pub enum GLFWmonitor {}
pub enum GLFWwindow {}

extern {
    pub fn glfwSetErrorCallback(cbfun: extern fn(c_int, *const c_char)) -> extern fn(c_int, *const c_char);

    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();

    pub fn glfwGetVersion(major: *mut c_int, minor: *mut c_int, rev: *mut c_int);
    pub fn glfwGetVersionString() -> *const c_char;

    pub fn glfwCreateWindow(width: c_int,
                            height: c_int,
                            title: *const c_char,
                            monitor: *const GLFWmonitor,
                            share: *const GLFWwindow) -> *mut GLFWwindow;
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);

    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut c_void;
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *mut c_void);

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
