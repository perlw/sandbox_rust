extern crate gcc;

use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("stb/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    fs::copy("stb/stb_image.h", "stb_image.c").unwrap();
    gcc::Build::new()
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .file("stb_image.c")
        .compile("stb_image");
}
