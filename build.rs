#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
fn main() {
    println!("cargo:rerun-if-changed=./extern");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-link-search=./extern/lib");
    Command::new("make")
        .current_dir("./extern")
        .status()
        .unwrap();
}
