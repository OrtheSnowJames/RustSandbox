use ffi::IsKeyDown;
use raylib::prelude::*;
use serde_json::Value;
use serde_json::json;

pub struct Movement {
    pub position: Vector2,
    pub speed: f32,
    pub width: i32,
    pub height: i32
}

impl Movement {
    pub fn update(&mut self, delta_time: f32) {
        let frame_speed = self.speed * delta_time * 40.0; 

        if unsafe { IsKeyDown(KeyboardKey::KEY_RIGHT as i32) || IsKeyDown(KeyboardKey::KEY_D as i32) } {
            self.position.x += frame_speed;
        }

        if unsafe { IsKeyDown(KeyboardKey::KEY_LEFT as i32) || IsKeyDown(KeyboardKey::KEY_A as i32) } {
            self.position.x -= frame_speed;
        }

        if unsafe { IsKeyDown(KeyboardKey::KEY_DOWN as i32) || IsKeyDown(KeyboardKey::KEY_S as i32) } {
            self.position.y += frame_speed;
        }

        if unsafe { IsKeyDown(KeyboardKey::KEY_UP as i32) || IsKeyDown(KeyboardKey::KEY_W as i32) } {
            self.position.y -= frame_speed;
        }
    }
}
// random advanced functions
pub fn calculate_distance(object1: Value, object2: Value) -> f32 {
    let x1 = object1["x"].as_f64().unwrap();
    let y1 = object1["y"].as_f64().unwrap();
    let x2 = object2["x"].as_f64().unwrap();
    let y2 = object2["y"].as_f64().unwrap();
    let distance = ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
    distance as f32
}

pub fn calculate_degrees(object1: Value, object2: Value) -> i32 {
    let x1 = object1["x"].as_f64().unwrap();
    let y1 = object1["y"].as_f64().unwrap();
    let x2 = object2["x"].as_f64().unwrap();
    let y2 = object2["y"].as_f64().unwrap();
    let degrees = ((y2 - y1).atan2(x2 - x1) * 180.0 / std::f64::consts::PI) as i32;
    degrees as i32
}

pub fn calculate_radians(object1: Value, object2: Value) -> f32 {
    let x1 = object1["x"].as_f64().unwrap();
    let y1 = object1["y"].as_f64().unwrap();
    let x2 = object2["x"].as_f64().unwrap();
    let y2 = object2["y"].as_f64().unwrap();
    let radians = (y2 - y1).atan2(x2 - x1) as f32;
    radians
}

pub struct line {
    point1: Vector2,
    point2: Vector2,
    point_on: Vector2
}

pub fn calculate_to(object1: Value, to_object: Value) -> line {
    let x1 = object1["x"].as_f64().unwrap();
    let y1 = object1["y"].as_f64().unwrap();
    let x2 = to_object["x"].as_f64().unwrap();
    let y2 = to_object["y"].as_f64().unwrap();
    let line = line {
        point1: Vector2::new(x1 as f32, y1 as f32),
        point2: Vector2::new(x2 as f32, y2 as f32),
        point_on: Vector2::new(x1 as f32, y1 as f32)
    };
    line
}

pub fn move_on_line(object1: Value, line: line, speed: f32) -> Vector2 {
    let x1 = object1["x"].as_f64().unwrap();
    let y1 = object1["y"].as_f64().unwrap();
    let x2 = line.point2.x;
    let y2 = line.point2.y;
    let distance = ((x2 - x1 as f32).powf(2.0) + (y2 - y1 as f32).powf(2.0)).sqrt();
    let x = speed * (x2 - x1 as f32) / distance as f32;
    let y = speed * (y2 - y1 as f32) / distance as f32;
    let position = Vector2::new(x as f32, y as f32);
    position
}

fn check_collision(player_rect: &Rectangle, rect: &Rectangle) -> bool {
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

pub fn line_done(object1: Value, line: line) -> bool {
    let mut returnable: bool = false;
    let ending_point: Vector2 = line.point2;
    let x1: i32 = object1["x"].as_i64().unwrap() as i32;
    let y1: i32 = object1["y"].as_i64().unwrap() as i32;
    let owidth: i32 = object1["width"].as_i64().unwrap() as i32;
    let oheight: i32 = object1["height"].as_i64().unwrap() as i32;
    if (check_collision(&Rectangle::new(x1 as f32, y1 as f32, owidth as f32, oheight as f32), &Rectangle::new(ending_point.x, ending_point.y, 1.0, 1.0))) {
        returnable = true;
    }
    returnable
}