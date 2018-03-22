extern crate bitcoinrs_sys;
extern crate libc;

mod vector;

use std::ffi::{CStr, CString};

use libc::c_char;

use bitcoinrs_sys::btc_get_peers_from_dns;

use self::vector::BtcVec;

pub fn get_peers_from_dns(seed: &str, port: i32, family: i32) -> Vec<String> {
    let mut vec: BtcVec<c_char> = BtcVec::new();
    vec.use_inner_vec(move |inner_vec| {
        let seed_bytes: *const u8 = seed.as_ptr();
        unsafe {
            let _count = btc_get_peers_from_dns(seed_bytes as _, inner_vec, port, family);
        }
        (inner_vec, ())
    });

    vec.iter()
        .map(|s| {
            let c_str: *const c_char = s;
            unsafe { CStr::from_ptr(c_str).to_owned().into_string().unwrap() }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_peers_from_dns() {
        let seed = "seed.bitcoin.jonasschnelli.ch";
        let port = 8333;
        let family = ::libc::AF_INET;
        let peers = get_peers_from_dns(seed, port, family);
        println!("{:?}", peers);
    }
}
