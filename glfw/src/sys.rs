use libc::{c_char, c_int};

pub const GLFW_FALSE: c_int = 0;
pub const GLFW_TRUE: c_int = 1;

pub const GLFW_NOT_INITIALIZED: c_int = 0x00010001;
pub const GLFW_NO_CURRENT_CONTEXT: c_int = 0x00010002;
pub const GLFW_INVALID_ENUM: c_int = 0x00010003;
pub const GLFW_INVALID_VALUE: c_int = 0x00010004;
pub const GLFW_OUT_OF_MEMORY: c_int = 0x00010005;
pub const GLFW_API_UNAVAILABLE: c_int = 0x00010006;
pub const GLFW_VERSION_UNAVAILABLE: c_int = 0x00010007;
pub const GLFW_PLATFORM_ERROR: c_int = 0x00010008;
pub const GLFW_FORMAT_UNAVAILABLE: c_int = 0x00010009;
pub const GLFW_NO_WINDOW_CONTEXT: c_int = 0x0001000A;

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

    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwPollEvents();
}
