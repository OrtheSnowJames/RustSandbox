use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::thread;
use async_std::task;
use serde_json::Value;
use serde_json::json;
use serde_json::from_str;
use serde_json::to_string;
use std::sync::{Arc, Mutex};

use crate::networking::AsyncTcpServer;
use crate::networking;

pub fn main() {
    let data_json = std::fs::read_to_string("data.json").expect("Failed to read data.json");
    let data: Value = serde_json::from_str(&data_json).expect("Failed to parse data.json");
    let settings = data["settings"].clone();
    let port_str = settings["PORT"].as_str().expect("Expected a string for PORT").to_string();
    let port = from_str::<u16>(&port_str).expect("Failed to parse PORT as u16");
    let ip_addr = networking::get_local_ip().expect("Failed to get local IP");

    let server = AsyncTcpServer::new(&format!("{}:{}", ip_addr, port), std::sync::Arc::new(|_| {}));
    
    println!("Server starting on {}:{}", ip_addr, port);
    
    // Create a game state that can be shared between connections
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
        server.run_with_messages(move |msg, mut stream| {
            let game_state = game_state.clone();
            async move {
                let msg_value: Value = serde_json::from_str(&msg).unwrap_or(json!({}));
                
                // Handle the message and update game state
                let response = {
                    let mut state = game_state.lock().unwrap();
                    match msg_value["type"].as_str() {
                        Some("update") => {
                            if let Some(player_id) = msg_value["player_id"].as_str() {
                                state["players"][player_id] = msg_value["data"].clone();
                                json!({"status": "ok", "type": "update_confirm"})
                            } else {
                                json!({"status": "error", "message": "missing player_id"})
                            }
                        },
                        _ => json!({"status": "error", "message": "unknown message type"})
                    }
                };

                AsyncTcpServer::send(&mut stream, &response.to_string()).await?;
                Ok(())
            }
        }).await.expect("Server failed to run");
    });
}