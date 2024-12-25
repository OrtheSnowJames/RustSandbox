use serde_json::Value;
use serde_json::json;

pub struct handle_read;

impl handle_read {
    pub fn handle_read_msg(message: &String) {
        // Parse the message
        let message_json: Value = serde_json::from_str(message).unwrap();
        // Check the message type (located like anywhere so whatever)
        if let Some(update_position) = message_json.get("update_position") {
            // Update the position
        }
    }
}
