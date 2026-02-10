use std::net::SocketAddr;


pub fn report_peer_connected(addr: SocketAddr) {
    println!("peer ({}, {}) connected", addr.ip(), addr.port());
}