use std::net::{TcpStream, ToSocketAddrs};
use std::result::Result;
use std::time::Duration;

use reqwest::Error;

pub fn get_pub_ip() -> Result<String, Error> {
    match reqwest::blocking::get("https://checkip.amazonaws.com") {
        Ok(r) => Result::Ok(String::from(r.text().unwrap().trim())),
        Err(e) => Err(e)
    }
}

pub fn can_connect(ip: &str,port: i32) -> bool {
    let addr_str = format!("{}:{}",ip,port);
    match String::from(addr_str).to_socket_addrs() {
        Ok(intoiter) => {
            let addr = intoiter.as_slice()[0];
            match TcpStream::connect_timeout(&addr,Duration::from_secs(5)) {
                Ok(_) => true,
                _ => false
            }
        },
        _=> false
    }
}
