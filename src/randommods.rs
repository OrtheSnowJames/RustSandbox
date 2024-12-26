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

use serde_json::{json, Value};

fn find_changes(old_value: &Value, new_value: &Value) -> Value {
    // Handle integer comparison
    if let (Some(old_int), Some(new_int)) = (old_value.as_i64(), new_value.as_i64()) {
        let difference = new_int - old_int;
        json!({
            "type": "int",
            "changed": {
                "difference": difference,
            },
            "final_value": new_int
        })
    }
    // Handle string comparison
    else if let (Some(old_str), Some(new_str)) = (old_value.as_str(), new_value.as_str()) {
        let added: Vec<char> = new_str.chars().filter(|&c| !old_str.contains(c)).collect();
        let removed: Vec<char> = old_str.chars().filter(|&c| !new_str.contains(c)).collect();
        
        json!({
            "type": "string",
            "added": added,
            "removed": removed,
            "final_value": new_str
        })
    }
    // Handle JSON comparison (for `serde_json::Value` types)
    else if old_value.is_object() && new_value.is_object() {
        let mut changes = serde_json::Map::new();

        // Compare each field in the JSON objects
        if let (Some(old_obj), Some(new_obj)) = (old_value.as_object(), new_value.as_object()) {
            for (key, new_field) in new_obj {
                if let Some(old_field) = old_obj.get(key) {
                    if old_field != new_field {
                        changes.insert(key.clone(), new_field.clone());
                    }
                } else {
                    // New field added in the new object
                    changes.insert(key.clone(), new_field.clone());
                }
            }

            // Check for removed fields (if any)
            for (key, old_field) in old_obj {
                if !new_obj.contains_key(key) {
                    changes.insert(key.clone(), Value::Null);
                }
            }
        }

        json!({
            "type": "json",
            "changed": changes,
            "final_value": new_value
        })
    }
    // If the values are of unsupported types
    else {
        json!({
            "error": "Unsupported type"
        })
    }
}
