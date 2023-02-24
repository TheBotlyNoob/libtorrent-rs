extern crate openssl_sys;

#[cxx::bridge(namespace = "libtorrent")]
pub mod ffi {
    unsafe extern "C++" {
        include!("libtorrent/version.hpp");

        fn version() -> *const c_char;
    }
}

#[test]
fn sanity_check() {
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(ffi::version()).to_str() },
        Ok("2.0.8.0")
    );
}
