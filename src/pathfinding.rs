use raylib::prelude::*;
use serde_json::Value;
use serde_json::json;

pub struct line {
    point1: Vector2,
    point2: Vector2,
    point_on: Vector2
}

pub fn draw_line(object1: Value, object2: Value) -> line {
    let x1: f64 = object1["x"].as_f64().unwrap();
    let y1: f64 = object1["y"].as_f64().unwrap();
    let x2: f64 = object2["x"].as_f64().unwrap();
    let y2: f64 = object2["y"].as_f64().unwrap();
    let line: line = line {
        point1: Vector2::new(x1 as f32, y1 as f32),
        point2: Vector2::new(x2 as f32, y2 as f32),
        point_on: Vector2::new(x1 as f32, y1 as f32)
    };
    line
}

