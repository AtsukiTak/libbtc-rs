use std::ffi::{CStr, CString};
use std::net::SocketAddr;
use std::str::FromStr;

use libc::c_char;

use libbtc_sys::btc_false;
use libbtc_sys::net::{btc_chainparams_main, btc_chainparams_regtest, btc_chainparams_test,
                      btc_get_peers_from_dns, btc_node_group_add_node, btc_node_group_new,
                      btc_node_new, btc_node_set_ipport, BtcNode, BtcNodeGroup};

use vector::BtcVec;

pub struct Node {
    inner: *mut BtcNode,
}

impl Node {
    pub fn new(addr: SocketAddr) -> Node {
        let node = unsafe { btc_node_new() };
        let addr_str = CString::new(format!("{}", addr)).unwrap();
        let raw_str = addr_str.into_raw();
        if unsafe { btc_node_set_ipport(node, raw_str) } == btc_false {
            println!("Error");
        }
        unsafe {
            drop(CString::from_raw(raw_str));
        }
        Node { inner: node }
    }
}

pub struct NodeGroup {
    inner: *mut BtcNodeGroup,
    nodes_: Vec<Node>, // Prevent Node to be dropped.
}

pub enum NetworkType {
    Main,
    Test,
    Regtest,
}

impl NodeGroup {
    pub fn new(net: NetworkType) -> NodeGroup {
        unsafe {
            let params = match net {
                NetworkType::Main => btc_chainparams_main,
                NetworkType::Test => btc_chainparams_test,
                NetworkType::Regtest => btc_chainparams_regtest,
            };

            NodeGroup {
                inner: btc_node_group_new(&params),
                nodes_: Vec::new(),
            }
        }
    }

    pub fn add_node(&mut self, node: Node) {
        unsafe {
            btc_node_group_add_node(self.inner, node.inner);
        }
        self.nodes_.push(node);
    }
}

/// Get peer's ip addresses from dns seed.
pub fn get_peers_from_dns(seed: &str, port: i32, family: i32) -> Vec<SocketAddr> {
    let mut vec: BtcVec<c_char> = BtcVec::new();
    vec.use_inner_vec(move |inner_vec| {
        let seed_bytes: *const u8 = seed.as_ptr();
        unsafe {
            let _cnt = btc_get_peers_from_dns(seed_bytes as _, inner_vec, port, family);
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

        // I'm not sure but if I comment below single line out,
        // peers.len() returns 0.
        // It maybe compiler optimization problem but I have no idea
        // what should I do.
        println!("{:?}", peers);

        assert!(peers.len() != 0);
    }
}
