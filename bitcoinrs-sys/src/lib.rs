extern crate libc;

use libc::{c_char, c_int, c_uchar, c_void, size_t, uint32_t, uint8_t};

pub type uint256 = [u8; 32];
pub type uint160 = [u8; 20];
pub type btc_bool = uint8_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct btc_dns_seed {
    domain: [c_char; 256],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BtcChainParams {
    pub chainname: [c_char; 32],
    pub b58prefix_pubkey_address: uint8_t,
    pub b58prefix_script_address: uint8_t,
    pub bech32_hrp: [c_char; 5],
    pub b58prefix_secret_address: uint8_t, // !private key
    pub b58prefix_bip32_privkey: uint32_t,
    pub b58prefix_bip32_pubkey: uint32_t,
    pub netmagic: [c_uchar; 4],
    pub genesisblockhash: uint256,
    pub default_port: c_int,
    pub dnsseeds: [btc_dns_seed; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector {
    pub data: *mut *mut c_void, /* array of pointers */
    pub len: size_t,            /* array element count */
    pub alloc: size_t,          /* allocated array elements */

    elem_free_f: fn(*mut c_void),
}

#[link(name = "btc", kind = "static")]
extern "C" {
    /* Bindings of memory.h */
    pub fn btc_malloc(size: size_t) -> *mut c_void;
    pub fn btc_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn btc_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn btc_free(ptr: *mut c_void);

    /* Bindings of vector.h */
    pub fn vector_new(res: size_t, free_f: fn(*mut c_void)) -> *mut Vector;
    pub fn vector_free(vec: *mut Vector, free_array: btc_bool);
    pub fn vector_add(vec: *mut Vector, data: *mut c_void) -> btc_bool;

    /* Bindings of net.h */
    pub fn btc_get_peers_from_dns(
        seed: *const c_char,
        ips_out: *mut Vector,
        port: c_int,
        family: c_int,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        unsafe {
            let vec = vector_new(10, |_v| ());
            let answer: *mut usize = &mut 42;
            let data = ::std::mem::transmute::<*mut usize, *mut c_void>(answer);
            assert!(vector_add(vec, data) == 1);

            let raw_data = vec.as_ref().unwrap().data.as_ref().unwrap()[0];
            assert_eq!(
                ::std::mem::transmute::<*mut c_void, *mut usize>(raw_data)
                    .as_ref()
                    .unwrap(),
                &42
            );
        };
    }
}
