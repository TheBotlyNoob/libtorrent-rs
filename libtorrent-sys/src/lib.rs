#[cxx::bridge(namespace = "libtorrent")]
mod ffi {
    unsafe extern "C++" {
        include!("libtorrent/version.hpp");

        fn version() -> *const char;
    }
}

#[test]
fn test_parse_magnet_uri() {
    unsafe {
        ffi::parse_magnet_uri("magnet:?xt=urn:btih:2d2d2d2d2d424547494e20");
    }
}
