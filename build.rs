use std::{fs::canonicalize, process::Command};

fn main() {
    let status = Command::new("make")
        .args(&["libindicators.a", "-C", "./tulipindicators"])
        .status()
        .expect("Failed to run make");

    let bindings = bindgen::Builder::default()
        .header("./tulipindicators/indicators.h")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
    assert!(status.success());
    // Assuming the .a file is named `libtulipindicators.a`
    println!("cargo:rustc-link-lib=static=indicators");
    println!("cargo:rustc-link-search=native=tulipindicators/");
    // Point to the directory containing the .a file
}
