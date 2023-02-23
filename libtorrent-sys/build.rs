use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let boost_path = std::env::var_os("BOOST_ROOT")
        .unwrap_or_else(std::ffi::OsString::new);
    let boost_path = std::path::Path::new(&boost_path);

    let mut conf = Config::new("vendor/libtorrent");

    conf.define("CMAKE_CXX_STANDARD", "17")
        .define("BUILD_SHARED_LIBS", "OFF") // static build
        .define("static_runtime", "ON") // static build
        .define("BOOST_ROOT", "");

    let dst = conf.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");

    println!("cargo:rustc-link-search=native={}", boost_path.display());

    if cfg!(target_os = "windows") {
        // link to winapi
        println!("cargo:rustc-link-lib=dylib=Iphlpapi");
        #[cfg(debug_assertions)]
        println!("cargo:rustc-link-lib=dylib=DbgHelp");
    }

    cxx_build::bridge("src/lib.rs")
        .include("vendor/libtorrent/include")
        .include(boost_path)
        .flag_if_supported("-std=c++17")
        .compile("libtorrent-sys");
}
