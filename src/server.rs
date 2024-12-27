use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::thread;
use async_std::task;
use serde_json::Value;
use serde_json::json;
use crate::randommods;
use serde_json::from_str;
use serde_json::to_string;
use std::sync::{Arc, Mutex};

use crate::networking::AsyncTcpServer;
use crate::networking;
use crate::handle_read::*;
use crate::networking::ClientConnections;

pub fn main() {
    let data_json = std::fs::read_to_string("data.json").expect("Failed to read data.json");
    let data: Value = serde_json::from_str(&data_json).expect("Failed to parse data.json");
    let settings = data["settings"].clone();
    let port_str = settings["PORT"].as_str().expect("Expected a string for PORT").to_string();
    let port = from_str::<u16>(&port_str).expect("Failed to parse PORT as u16");
    let ip_addr = randommods::get_external_ipv4().expect("Failed to get local IP");
    let server = AsyncTcpServer::new(&format!("{}:{}", ip_addr, port), std::sync::Arc::new(|_| {}));
    
    println!("Server starting on {}:{}", ip_addr, port);
    
    // Create a game state that can be shared between connections
    let clients = Arc::new(Mutex::new(ClientConnections::new()));
    let game_state = Arc::new(Mutex::new(json!({
        "room1": {
            "objects": [
                //square that player moves on
                {"x": 0, "y": 0, "width": 1000, "height": 1000, "id": 0},
            ],
            "players": [],
            "npcs": [],
            "roomID": 1
        },
        "room2": {
            "objects": [
                //square that player moves on
                {"x": 0, "y": 0, "width": 1000, "height": 1000, "id": 0},
            ],
            "players": [],
            "npcs": [],
            "roomID": 2
        },
    })));

    task::block_on(async move {
        server.run_with_messages(move |msg, stream| {
            let game_state = game_state.clone();
            let clients = clients.clone();
            async move {
                let client_id = AsyncTcpServer::get_socket_id(&stream);
                clients.lock().unwrap().add_client(client_id as u32, stream.clone());
                
                handle_read_server(&msg, game_state.clone(), client_id as u32, &mut clients.lock().unwrap());
                Ok(())
            }
        }).await.expect("Server failed to run");
    });
}