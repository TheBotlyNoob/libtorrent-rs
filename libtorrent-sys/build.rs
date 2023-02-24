use std::env;

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut conf = Config::new("vendor/libtorrent");

    conf.define("CMAKE_CXX_STANDARD", "17")
        .define("BUILD_SHARED_LIBS", "OFF") // static build
        .define("static_runtime", "ON") // static build
        .define("BOOST_ROOT", "")
        .static_crt(false);

    let small = ["s", "z"].contains(&env::var("OPT_LEVEL").unwrap_or_default().as_str());
    conf.profile(if small { "MinSizeRel" } else { "Release" });

    let dst = conf.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");

    if cfg!(target_os = "windows") {
        // link to winapi
        println!("cargo:rustc-link-lib=dylib=Iphlpapi");
    }

    let boost_path = env::var_os("BOOST_ROOT").unwrap_or_else(std::ffi::OsString::new);
    let boost_path = std::path::Path::new(&boost_path);

    cxx_build::bridge("src/lib.rs")
        .include("vendor/libtorrent/include")
        .include(boost_path)
        .flag_if_supported("-std=c++17")
        .compile("libtorrent-sys");
}
