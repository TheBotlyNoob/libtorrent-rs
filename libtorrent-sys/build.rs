use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut conf = Config::new("vendor/libtorrent");

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

    let boost_path = std::env::var_os("BOOST_ROOT").unwrap_or_else(std::ffi::OsString::new);
    let boost_path = std::path::Path::new(&boost_path);

    cxx_build::bridge("src/lib.rs")
        .include("vendor/libtorrent/include")
        .include(boost_path)
        .flag_if_supported("-std=c++17")
        .compile("libtorrent-sys");
}
