extern crate cmake;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("glfw/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    let target = env::var("TARGET").unwrap();

    let mut config = cmake::Config::new("glfw");
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("GLFW_INSTALL", "OFF");

    if target.contains("windows") {
        if target.contains("msvc") {
            config.generator("NMake Makefiles");
        }
        config.define("USE_MSVC_RUNTIME_LIBRARY_DLL", "OFF");
    }

    config.build_target("");
    let dest = config.build();

    println!("cargo:rustc-link-lib=static=glfw3");
    println!(
        "cargo:rustc-link-search=native={}/build/src",
        dest.display()
    );
    println!("cargo:include=cargo:root/glfw/include");
}
