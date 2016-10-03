extern crate pkg_config;

use std::process::Command;

fn main() {
    // Temporarily hard-code the linker params
    println!("cargo:rustc-link-lib=sgutils2");
}
