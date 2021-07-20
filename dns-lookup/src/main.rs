#![allow(unused)]

use std::io;
use std::env;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = env::args().collect();

    for hn in args[1..].iter() {

        let host = (hn.clone(), 0);
        let address_iter = host.to_socket_addrs().unwrap();

        for socket in address_iter {        

            let ip = socket.ip();

            let ver = if ip.is_ipv4() { "IPv4" } else { "IPv6" };

            println!("{} {} {}", hn, ver, ip);

        }
    }
}
