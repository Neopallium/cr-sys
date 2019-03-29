// build.rs

extern crate cc;

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {  
    if cfg!(feature = "guest") {
        build_guest();
    } else {
        build_host();
    }
}

fn build_guest() {  
    println!("cargo:rustc-cfg=guest");
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // just client-side types
        .header("vendor/cr.h")
        // Enums
        .rustified_enum("cr_op")
        .whitelist_type("cr_op")
        .rustified_enum("cr_failure")
        .whitelist_type("cr_failure")
        // structs
        .whitelist_type("cr_plugin")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/guest.c")
        .flag("-Wno-unused-parameter")
        .compile("cr");
}

fn build_host() {  
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // enable CR_HOST
        .header("src/host.cpp")
        // compile as c++
        .clang_arg("-x").clang_arg("c++")
        // Host-side functions
        .whitelist_function("cr_plugin_load")
        .whitelist_function("cr_plugin_update")
        .whitelist_function("cr_plugin_close")
        .whitelist_function("wrap_cr_set_temporary_path")
        // Enums
        .rustified_enum("cr_op")
        .whitelist_type("cr_op")
        .rustified_enum("cr_failure")
        .whitelist_type("cr_failure")
        // structs
        .whitelist_type("cr_plugin")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .cpp(true)
        .file("src/host.cpp")
        .flag("-Wno-unused-parameter")
        .compile("cr");
}
