use std::{env, path::PathBuf};

use cmake::Config;

fn main() {
    let mut conf = Config::new("vendor/libtorrent");

    #[cfg(not(debug_assertions))]
    conf.define("CMAKE_BUILD_TYPE", "Release");

    conf.define("CMAKE_CXX_STANDARD", "14")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("static_runtime", "ON"); // static build

    let dst = conf.build();

    // println!("cargo:rustc-link-lib=static=boost");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");

    let incl = dst.join("include");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(incl.join("libtorrent").join("fwd.hpp").to_string_lossy())
        // include dir
        .clang_args([&format!("-I{}", incl.display()), "-xc++", "-std=c++14"])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
