extern crate pkg_config;

use std::process::Command;

fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    if pkg_config::find_library("libsgutils2").is_ok() {
        return
    }
}
