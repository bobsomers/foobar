extern crate cc;

use std::env;
use std::collections::HashMap;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let mut config: HashMap<_, _> = [
        ("_GLFW_X11", "0"),
        ("_GLFW_WIN32", "0"),
        ("_GLFW_COCOA", "0"),
        ("_GLFW_WAYLAND", "0"),
        ("_GLFW_MIR", "0"),
        ("_GLFW_BUILD_DLL", "0"),
        ("_GLFW_VULKAN_STATIC", "0"),
        ("_GLFW_USE_HYBRID_HPG", "0"),
        ("_GLFW_HAS_XF86VM", "0"),
        ("_GLFW_USE_CHDIR", "0"),
        ("_GLFW_USE_MENUBAR", "0"),
        ("_GLFW_USE_RETINA", "0"),
    ].iter().cloned().collect();

    if target_os == "macos" {
        *config.entry("_GLFW_COCOA").or_insert("0") = "1";
        *config.entry("_GLFW_USE_CHDIR").or_insert("0") = "1";
        *config.entry("_GLFW_USE_MENUBAR").or_insert("0") = "1";
        *config.entry("_GLFW_USE_RETINA").or_insert("0") = "1";
    }

    // TODO: Linux/Windows options

    let common_srcs = vec![
        "glfw-3.2.1/src/context.c",
        "glfw-3.2.1/src/init.c",
        "glfw-3.2.1/src/input.c",
        "glfw-3.2.1/src/monitor.c",
        "glfw-3.2.1/src/vulkan.c",
        "glfw-3.2.1/src/window.c",
    ];

    let platform_srcs = match target_os.as_ref() {
        "macos" => vec![
            "glfw-3.2.1/src/cocoa_init.m",
            "glfw-3.2.1/src/cocoa_joystick.m",
            "glfw-3.2.1/src/cocoa_monitor.m",
            "glfw-3.2.1/src/cocoa_window.m",
            "glfw-3.2.1/src/cocoa_time.c",
            "glfw-3.2.1/src/posix_tls.c",
            "glfw-3.2.1/src/nsgl_context.m",
        ],
        // TODO: Linux/Windows platform sources.
        _ => panic!("Target OS '{}' not supported by GLFW.", target_os),
    };

    let mut cc_builder = cc::Build::new();
    for (key, value) in &config {
        cc_builder.define(key, Some(*value));
    }
    cc_builder.include("glfw-3.2.1/include")
        .include("glfw-3.2.1/src")
        .files(&common_srcs)
        .files(&platform_srcs)
        .warnings(false)
        .flag_if_supported("-Wno-deprecated-declarations")
        .compile("glfw");
}
