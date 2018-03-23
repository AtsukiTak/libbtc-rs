use libc::{c_char, c_int, c_uchar, c_void, size_t, uint32_t, uint8_t};

use {Vector, uint256};

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

#[link(name = "btc", kind = "static")]
extern "C" {
    /* Bindings of net.h */
    pub fn btc_get_peers_from_dns(
        seed: *const c_char,
        ips_out: *mut Vector,
        port: c_int,
        family: c_int,
    ) -> c_int;
}
