use serde_json::Value;
use serde_json::json;
use crate::randommods::*;
use std::sync::{Arc, Mutex};

pub struct handle_read;

impl handle_read {
    fn get_game_handler(game: &mut Value, message_json: &String) -> Value {
        // Update the game state
        let json_value: Value = serde_json::from_str(message_json).unwrap_or_else(|_| Value::Null);
        *game = json_value
            .as_object()
            .and_then(|obj| obj.get("get_game").cloned())
            .unwrap_or_else(|| Value::Null);
        game.clone()
    }

    fn get_player_handler(game: &mut Value, message_json: &Value) {
        // Locate the correct room and check/add the player
        if let Value::Object(rooms) = game {
            for (_, room) in rooms.iter_mut() {
                if let Some(players) = room.get_mut("players").and_then(|p| p.as_array_mut()) {
                    let player_id = message_json["get_player"]["id"].clone();
                    let mut player_set = false;

                    for player in players.iter_mut() {
                        if player["id"] == player_id {
                            *player = message_json["get_player"].clone();
                            player_set = true;
                            break;
                        }
                    }

                    if !player_set {
                        players.push(message_json["get_player"].clone());
                    }
                }
            }
        }
    }

    fn update_position(game: &mut Value, message_json: &Value) {
        // Locate the correct room and update player position
        if let Value::Object(rooms) = game {
            for (_, room) in rooms.iter_mut() {
                if let Some(players) = room.get_mut("players").and_then(|p| p.as_array_mut()) {
                    let player_id = message_json["update_position"]["id"].clone();

                    for player in players.iter_mut() {
                        if player["id"] == player_id {
                            // Update position-related fields
                            if let Some(x) = message_json["update_position"].get("x") {
                                player["x"] = x.clone();
                            }
                            if let Some(y) = message_json["update_position"].get("y") {
                                player["y"] = y.clone();
                            }
                            if let Some(width) = message_json["update_position"].get("width") {
                                player["width"] = width.clone();
                            }
                            if let Some(height) = message_json["update_position"].get("height") {
                                player["height"] = height.clone();
                            }
                            if let Some(sprite_state) = message_json["update_position"].get("sprite_state") {
                                player["sprite_state"] = sprite_state.clone();
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    fn update_npc_position(game: &mut Value, message_json: &Value) {
        // Locate the correct room and update NPC position
        if let Value::Object(rooms) = game {
            for (_, room) in rooms.iter_mut() {
                if let Some(npcs) = room.get_mut("npcs").and_then(|p| p.as_array_mut()) {
                    let npc_id = message_json["update_npc_position"]["id"].clone();

                    for npc in npcs.iter_mut() {
                        if npc["id"] == npc_id {
                            // Update position-related fields
                            if let Some(x) = message_json["update_npc_position"].get("x") {
                                npc["x"] = x.clone();
                            }
                            if let Some(y) = message_json["update_npc_position"].get("y") {
                                npc["y"] = y.clone();
                            }
                            if let Some(width) = message_json["update_npc_position"].get("width") {
                                npc["width"] = width.clone();
                            }
                            if let Some(height) = message_json["update_npc_position"].get("height") {
                                npc["height"] = height.clone();
                            }
                            if let Some(sprite_state) = message_json["update_npc_position"].get("sprite_state") {
                                npc["sprite_state"] = sprite_state.clone();
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    // Main checking
    pub fn handle_read_msg(message: &String, game: Arc<Mutex<Value>>) {
        // Parse the message
        let message_json: Value = serde_json::from_str(message).unwrap_or_else(|_| Value::Null);

        // Lock the game state
        let mut game = game.lock().unwrap();

        // Check the message type and dispatch to handlers
        if json_contains(&message_json, "get_game") {
            *game = handle_read::get_game_handler(&mut *game, message);
        }

        if json_contains(&message_json, "get_player") {
            handle_read::get_player_handler(&mut *game, &message_json);
        }

        if json_contains(&message_json, "update_position") {
            handle_read::update_position(&mut *game, &message_json);
        }

        if json_contains(&message_json, "update_npc_position") {
            handle_read::update_npc_position(&mut *game, &message_json);
        }
    }
}
