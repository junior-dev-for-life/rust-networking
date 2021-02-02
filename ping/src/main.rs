use std::net::{ToSocketAddrs, SocketAddr};

fn main() {
    let mut addrs_iter = "www.google.com:80".to_socket_addrs().unwrap();

    match addrs_iter.next() {
        Some(c) => {
            println!("{}", c);
        },
        None => ()
    }
}
