//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

extern crate bindgen;
extern crate cc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    cc::Build::new()
        .file("./3parts/ws2812b/src/driver_ws2812b.c")
        .shared_flag(false)
        .compile("libws2812b.a");

    println!("cargo:rustc-link-lib=static=ws2812b");
    // println!("cargo:rerun-if-changed=wrapper.h");

    // let bindings = bindgen::Builder::default()
    //     .header("./3parts/wrapper.h")
    //     .ctypes_prefix("cty")
    //     .use_core()
    //     .clang_arg("-target thumbv6m-none-eabi")
    //     .clang_arg("--sysroot=/usr/arm-none-eabi")
    // .generate()
    // .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file(out.join("bindings.rs"))
    //     .expect("Couldn't write bindings.");
}
