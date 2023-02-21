use std::{env, path::Path};

use cmake::Config;

fn main() -> miette::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

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

    let boost_path = env::var("BOOST_ROOT").expect("BOOST_ROOT not set");
    let boost_path = Path::new(&boost_path);

    let mut b =
        autocxx_build::Builder::new("src/lib.rs", [&dst.join("include"), boost_path]).build()?;
    // This assumes all your C++ bindings are in lib.rs
    b.flag_if_supported("-std=c++14").compile("autocxx"); // arbitrary library name, pick anything
    Ok(())
}
