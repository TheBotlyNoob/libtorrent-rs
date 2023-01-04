use cmake::Config;

fn main() {
    let mut conf = Config::new("include/libtorrent");

    #[cfg(not(debug_assertions))]
    conf.define("CMAKE_BUILD_TYPE", "Release");

    conf.define("CMAKE_CXX_STANDARD", "17");

    let dst = conf.build().join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");
}
