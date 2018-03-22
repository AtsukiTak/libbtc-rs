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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NODE_STATE {
    NODE_CONNECTING = (1 << 0),
    NODE_CONNECTED = (1 << 1),
    NODE_ERRORED = (1 << 2),
    NODE_TIMEOUT = (1 << 3),
    NODE_HEADERSYNC = (1 << 4),
    NODE_BLOCKSYNC = (1 << 5),
    NODE_MISSBEHAVED = (1 << 6),
    NODE_DISCONNECTED = (1 << 7),
    NODE_DISCONNECTED_FROM_REMOTE_PEER = (1 << 8),
}

#[link(name = "btc", kind = "static")]
extern "C" {
    pub fn vector_new(res: size_t, free_f: fn(*mut c_void)) -> *mut Vector;
    pub fn vector_free(vec: *mut Vector, free_array: btc_bool);
    pub fn vector_add(vec: *mut Vector, data: *mut c_void) -> btc_bool;

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
