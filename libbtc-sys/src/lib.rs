#![allow(non_camel_case_types)]

extern crate libc;

pub mod net;
pub mod vector;

use libc::{c_void, size_t, uint8_t};

pub type uint256 = [u8; 32];
pub type uint160 = [u8; 20];
pub type btc_bool = uint8_t;

#[link(name = "btc", kind = "static")]
extern "C" {
    /* Bindings of memory.h */
    pub fn btc_malloc(size: size_t) -> *mut c_void;
    pub fn btc_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn btc_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn btc_free(ptr: *mut c_void);
}
