use raylib::prelude::*;
use crate::movement::Movement;

pub fn check_point_collision(player: &Movement, rect: &Rectangle) -> Option<i32> {
    let player_rect = Rectangle {
        x: player.position.x,
        y: player.position.y,
        width: player.width as f32,
        height: player.height as f32,
    };

    if player_rect.y < rect.y {
        Some(1) //Top collision
    } else if player_rect.x + player_rect.width > rect.x + rect.width {
        Some(2) //Right collision
    } else if player_rect.y + player_rect.height > rect.y + rect.height {
        Some(3) //Bottom collision
    } else if player_rect.x < rect.x {
        Some(4) //Left collision
    } else {
        None
    }
}

pub fn check_collision(player: &Movement, rect: &Rectangle) -> bool {
    let player_rect = Rectangle {
        x: player.position.x,
        y: player.position.y,
        width: player.width as f32,
        height: player.height as f32,
    };

    if player_rect.x < rect.x + rect.width
        && player_rect.x + player_rect.width > rect.x
        && player_rect.y < rect.y + rect.height
        && player_rect.y + player_rect.height > rect.y
    {
        true
    } else {
        false
    }
}

fn push_inward(player: &mut Movement, objectrect: &Rectangle, wall: i32) {
    match wall {
        1 => {
            if player.position.y < objectrect.y {
                player.position.y = objectrect.y;
            } else if player.position.y + player.height as f32 > objectrect.y + objectrect.height {
                player.position.y = objectrect.y + objectrect.height - player.height as f32;
            }
        }
        2 => {
            if player.position.x + player.width as f32 > objectrect.x + objectrect.width {
                player.position.x = objectrect.x + objectrect.width - player.width as f32;
            } else if player.position.x < objectrect.x {
                player.position.x = objectrect.x;
            }
        }
        3 => {
            if player.position.y + player.height as f32 > objectrect.y + objectrect.height {
                player.position.y = objectrect.y + objectrect.height - player.height as f32;
            } else if player.position.y < objectrect.y {
                player.position.y = objectrect.y;
            }
        }
        4 => {
            if player.position.x < objectrect.x {
                player.position.x = objectrect.x;
            } else if player.position.x + player.width as f32 > objectrect.x + objectrect.width {
                player.position.x = objectrect.x + objectrect.width - player.width as f32;
            }
        }
        _ => {}
    }
}

pub fn do_get_collision(player: &mut Movement, object: &mut serde_json::Value) {
    let objectrect = Rectangle {
        x: object["x"].as_f64().unwrap_or(0.0) as f32,
        y: object["y"].as_f64().unwrap_or(0.0) as f32,
        width: object["width"].as_f64().unwrap_or(0.0) as f32,
        height: object["height"].as_f64().unwrap_or(0.0) as f32,
    };
    let walltouched = check_point_collision(player, &objectrect);
    if let Some(wall) = walltouched {
        match wall {
            1 => player.position.y = objectrect.y,
            2 => player.position.x = objectrect.x + objectrect.width - player.width as f32,
            3 => player.position.y = objectrect.y + objectrect.height - player.height as f32,
            4 => player.position.x = objectrect.x,
            _ => {}
        }
    }
}

pub fn reverse_do_get_collision(player: &mut Movement, object: &mut serde_json::Value) {
    let objectrect = Rectangle {
        x: object["x"].as_f64().unwrap_or(0.0) as f32,
        y: object["y"].as_f64().unwrap_or(0.0) as f32,
        width: object["width"].as_f64().unwrap_or(0.0) as f32,
        height: object["height"].as_f64().unwrap_or(0.0) as f32,
    };
    let walltouched = check_point_collision(player, &objectrect);
    if let Some(wall) = walltouched {
        push_inward(player, &objectrect, wall);
    }
}