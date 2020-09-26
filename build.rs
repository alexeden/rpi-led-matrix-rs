extern crate cc;

#[cfg(not(target_os = "linux"))]
fn build_extern() {}

#[cfg(target_os = "linux")]
fn build_extern() {
    let compilation = cc::Build::new()
        // .cpp(true)
        .flag("-Wno-missing-field-initializers")
        .flag("-Wno-unused-parameter")
        .include("./extern/include")
        .files(&[
            "./extern/lib/thread.cc",
            "./extern/lib/pixel-mapper.cc",
            "./extern/lib/options-initialize.cc",
            "./extern/lib/multiplex-mappers.cc",
            "./extern/lib/led-matrix-c.cc",
            "./extern/lib/led-matrix.cc",
            "./extern/lib/graphics.cc",
            "./extern/lib/gpio.cc",
            "./extern/lib/framebuffer.cc",
            "./extern/lib/content-streamer.cc",
            "./extern/lib/bdf-font.cc",
            "./extern/lib/hardware-mapping.c",
        ])
        .try_compile("rgbmatrix");

    if let Err(e) = compilation {
        println!("cargo:warning={:?}", e);
    }
}

fn main() {
    // println!("cargo:rerun-if-changed=./extern");
    println!("cargo:rustc-flags=-l dylib=stdc++");
    build_extern();
}
