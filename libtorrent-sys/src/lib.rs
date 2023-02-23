extern crate openssl_sys;

#[cxx::bridge(namespace = "libtorrent")]
pub mod ffi {
    unsafe extern "C++" {
        include!("libtorrent/version.hpp");

        fn version() -> *const c_char;
    }
}

#[test]
fn test_version() {
    panic!("version: {}", unsafe { std::ffi::CStr::from_ptr(ffi::version()).to_str().unwrap() });
}
