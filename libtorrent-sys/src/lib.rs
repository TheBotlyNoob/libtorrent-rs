//! Raw Rust bindings to the `libtorrent` C-library.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

extern crate link_cplusplus;
extern crate openssl_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
