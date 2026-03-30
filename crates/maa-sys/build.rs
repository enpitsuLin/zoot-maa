//! # Build script
//!

use std::env;
use std::path::PathBuf;
extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=../../maa-include/include/AsstCaller.h");

    let out_dir = env::var("OUT_DIR").unwrap();

    // Use the header file from maa-include submodule
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let maa_header_path = PathBuf::from(&manifest_dir)
        .join("../../maa-include/include/AsstCaller.h")
        .canonicalize()
        .expect("Failed to find maa-include/include/AsstCaller.h");

    let bindings = bindgen::Builder::default()
        .header(maa_header_path.to_str().unwrap())
        .dynamic_library_name("MaaCore")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
