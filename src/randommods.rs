use async_std::io::prelude::*;
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawSocket;
use async_std::net::TcpStream as AsyncTcpStream;
use async_std::os::unix::io::AsRawFd as AsyncRawFd;
#[cfg(windows)]
use async_std::os::windows::io::AsRawSocket as AsyncRawSocket;

pub fn json_contains(json: &serde_json::Value, key: &str) -> bool {
    json.get(key).is_some()
}

pub fn get_socket_id(stream: &AsyncTcpStream) -> usize {
    #[cfg(unix)]
    {
        stream.as_raw_fd() as usize
    }
    #[cfg(windows)]
    {
        stream.as_raw_socket() as usize
    }
}