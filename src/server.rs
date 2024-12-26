use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::thread;
use async_std::task;
use serde_json::Value;
use serde_json::json;
use serde_json::from_str;
use crate::networking::AsyncTcpServer;
use crate::networking;

pub fn main() {
    let data_json = std::fs::read_to_string("data.json").expect("Failed to read data.json");
    let data: Value = serde_json::from_str(&data_json).expect("Failed to parse data.json");
    let settings = data["settings"].clone();
    let port_str = settings["PORT"].as_str().expect("Expected a string for PORT").to_string();
    let port = from_str::<u16>(&port_str).expect("Failed to parse PORT as u16");
    let ip_addr = networking::get_local_ip().expect("Failed to get local IP");

    // start/create cli server
    let handler = std::sync::Arc::new(|stream: async_std::net::TcpStream| {
        task::block_on(async move {
            task::spawn(async move {
                // handle the stream asynchronously
            }).await;
        })
    });

    let server = AsyncTcpServer::new(&format!("{}:{}", ip_addr, port), handler);
}