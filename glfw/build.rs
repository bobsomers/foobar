extern crate cc;

use std::env;
use std::collections::HashMap;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Platform defines include:
    //      _GLFW_X11
    //      _GLFW_WIN32
    //      _GLFW_COCOA
    //      _GLFW_WAYLAND
    //      _GLFW_MIR
    //      _GLFW_BUILD_DLL
    //      _GLFW_VULKAN_STATIC
    //      _GLFW_USE_HYRBID_HPG
    //      _GLFW_HAS_XF86VM
    //      _GLFW_USE_CHDIR
    //      _GLFW_USE_MENUBAR
    //      _GLFW_USE_RETINA

    let mut config = HashMap::new();

    if target_os == "macos" {
        config.insert("_GLFW_COCOA", "1");
        config.insert("_GLFW_USE_CHDIR", "1");
        config.insert("_GLFW_USE_MENUBAR", "1");
        config.insert("_GLFW_USE_RETINA", "1");
    } else if target_os == "linux" {
        config.insert("_GLFW_X11", "1");
        config.insert("_GLFW_HAS_XF86VM", "1");
    }
    // TODO: Windows options

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
        "linux" => vec![
            "glfw-3.2.1/src/x11_init.c",
            "glfw-3.2.1/src/x11_monitor.c",
            "glfw-3.2.1/src/x11_window.c",
            "glfw-3.2.1/src/xkb_unicode.c",
            "glfw-3.2.1/src/linux_joystick.c",
            "glfw-3.2.1/src/posix_time.c",
            "glfw-3.2.1/src/posix_tls.c",
            "glfw-3.2.1/src/glx_context.c",
            "glfw-3.2.1/src/egl_context.c",
        ],
        // TODO: Windows platform sources.
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

    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=OpenGL");
    } else if target_os == "linux" {
        println!("cargo:rustc-link-lib=dylib=rt");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=Xrandr");
        println!("cargo:rustc-link-lib=dylib=Xinerama");
        println!("cargo:rustc-link-lib=dylib=Xxf86vm");
        println!("cargo:rustc-link-lib=dylib=Xcursor");
        println!("cargo:rustc-link-lib=dylib=X11");
    }
}
