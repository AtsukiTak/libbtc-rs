use libc::{c_char, c_int, c_uchar, c_uint, c_void, sockaddr, uint32_t, uint64_t, uint8_t};

use {btc_bool, uint256};
use vector::Vector;

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

pub enum BufferEvent {}
pub enum ConstBuffer {}
pub enum Event {}
pub enum EventBase {}
pub enum BtcP2PMsgHdr {}
pub enum cstring {} // Should implement later.

pub struct BtcNodeGroup {
    ctx: *mut c_void, /* flexible context usefull in conjunction with the callbacks */
    event_base: *mut EventBase,
    nodes: *mut Vector, /* the groups nodes */
    clientstr: [c_char; 1024],
    desired_amount_connected_nodes: c_int,
    chainparams: *const BtcChainParams,

    /* callbacks */
    log_write_cb: extern "C" fn(*const c_char, ...) -> c_int, /* log callback, default=printf */
    parse_cmd_cb: extern "C" fn(*mut BtcNode, *mut BtcP2PMsgHdr, *mut ConstBuffer) -> btc_bool,
    postcmd_cb: extern "C" fn(*mut BtcNode, *mut BtcP2PMsgHdr, *mut ConstBuffer),
    node_connection_state_changed_cb: extern "C" fn(*mut BtcNode),
    should_connect_to_more_nodes_cb: extern "C" fn(*mut BtcNode) -> btc_bool,
    handshake_done_cb: extern "C" fn(*mut BtcNode),
    periodic_timer_cb: extern "C" fn(*mut BtcNode, *mut uint64_t) -> btc_bool, // return false will cancle the internal logic
}

#[repr(C)]
pub struct BtcNode {
    addr: sockaddr,
    event_bev: *mut BufferEvent,
    timer_event: *mut Event,
    nodegroup: *mut BtcNodeGroup,
    nodeid: c_int,
    lastping: uint64_t,
    time_started_con: uint64_t,
    time_last_request: uint64_t,
    last_requested_inv: uint256,

    recvBuffer: *mut cstring,
    nonce: uint64_t,
    services: uint64_t,
    state: uint32_t,
    missbehavescore: c_int,
    version_handshake: btc_bool,

    bestknownheight: c_uint,

    hints: uint32_t, /* can be use for user defined state */
}

#[link(name = "btc", kind = "static")]
extern "C" {
    /* ======================================= */
    /* NODES                                   */
    /* ======================================= */

    pub fn btc_node_new() -> *mut BtcNode;
    pub fn btc_node_free(node: *mut BtcNode);

    /* set the nodes ip address and port (ipv4 or ipv6)*/
    pub fn btc_node_set_ipport(node: *mut BtcNode, ipport: *const c_char) -> btc_bool;

    /* ======================================= */
    /* NODE GROUPS                             */
    /* ======================================= */

    pub fn btc_node_group_new(params: *const BtcChainParams) -> *mut BtcNodeGroup;
    pub fn btc_node_group_free(group: *mut BtcNodeGroup);

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
    fn btc_node_new_and_free_should_not_panic() {
        unsafe {
            let node = btc_node_new();
            assert!(!node.is_null());
            btc_node_free(node);
        }
    }
}
