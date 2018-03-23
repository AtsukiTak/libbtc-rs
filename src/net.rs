use std::ffi::CStr;
use std::net::SocketAddr;
use std::str::FromStr;

use libc::c_char;

use libbtc_sys::net::btc_get_peers_from_dns;

use vector::BtcVec;

/// Get peer's ip addresses from dns seed.
pub fn get_peers_from_dns(seed: &str, port: i32, family: i32) -> Vec<SocketAddr> {
    let mut vec: BtcVec<c_char> = BtcVec::new();
    vec.use_inner_vec(move |inner_vec| {
        let seed_bytes: *const u8 = seed.as_ptr();
        unsafe {
            let _cnt = btc_get_peers_from_dns(seed_bytes as _, inner_vec, port, family);
            println!("count : {}", _cnt);
        }
        (inner_vec, ())
    });

    vec.iter()
        .map(|s| {
            /* "s" is &c_char */
            let c_str: *const c_char = s;
            let s = unsafe { CStr::from_ptr(c_str).to_str().unwrap() };
            SocketAddr::from_str(s).unwrap()
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
        assert!(peers.len() == 24);
    }
}
