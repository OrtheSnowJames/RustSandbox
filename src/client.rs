use raylib::prelude::*;
use raylib_interactive;
use raylib_interactive::button::Button;
use serde_json::Value;
use serde_json::json;
use std::env;
use crate::movement;
use crate::collision;

pub fn main() {
    let mut window_length: i32 = env::var("RSWINDOW_LENGTH").unwrap_or_else(|_| "1000".to_string()).parse().unwrap();
    let mut window_height: i32 = env::var("RSWINDOW_HEIGHT").unwrap_or_else(|_| "1000".to_string()).parse().unwrap();
    if window_length < 1000 || window_height < 1000 {
        //no panic just set to 1000
        env::set_var("RSWINDOW_LENGTH", "1000");
        env::set_var("RSWINDOW_HEIGHT", "1000");
        window_length = env::var("RSWINDOW_LENGTH").unwrap_or_else(|_| "1000".to_string()).parse().unwrap();
        window_height = env::var("RSWINDOW_HEIGHT").unwrap_or_else(|_| "1000".to_string()).parse().unwrap();
    }
    let (mut rl, thread) = raylib::init()
        .size(window_length, window_height)
        .title("raylib thing")
        .build();
    println!("width = {}", window_length);
    println!("height = {}", window_height);
    rl.set_target_fps(60);
    //init stuff here
    let mut movement = movement::Movement {
        position: Vector2::new(400.0, 250.0),
        speed: 5.0,
        width: 50,
        height: 50,
    };
    //define game here
    let room_in: i32 = 1;
    let whole_room_in: String = "room".to_string() + &room_in.to_string();
    let game: Value = json!({
        "room1": {
            "objects": [
                //square that player moves on
                {"x": 0, "y": 0, "width": 1000, "height": 1000, "objID": 0},
            ]
        },
    });
    let objects_interpret_inside: Value = json!([
        0
    ]);
    let mut button = Button::new(((window_length as i32) / 2) as f32, ((window_height as i32) / 2) as f32, 100 as f32, 50 as f32, "position");
    button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::BLACK);
    button.set_font_size(10);
    //loop
    
    while !rl.window_should_close() {
        button.update(&mut rl);
        if button.is_clicked(& mut rl) {movement.position.x = 400.0; movement.position.y = 250.0;}
        movement.update(rl.get_frame_time());
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        //get collisions
        if let Some(objects) = game[whole_room_in.clone()]["objects"].as_array() {
            for object in objects {
                if objects_interpret_inside.as_array().unwrap().contains(&object["objID"]) {
                    //treat like inside object
                    collision::reverse_do_get_collision(&mut movement, &mut object.clone());
                } else {
                    //treat like outside object
                    collision::do_get_collision(&mut movement, &mut object.clone());
                }
            }
        }
        //drawing code seperate line here
        d.clear_background(Color::WHITE);
        d.draw_rectangle(1, 1, 1000, 1000, Color::GRAY);
        d.draw_rectangle(
            movement.position.x as i32,
            movement.position.y as i32,
            movement.width,
            movement.height,
            Color::RED,
        );
        button.draw(&mut d);
    }
}