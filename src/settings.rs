use raylib::prelude::*;
use raylib_interactive;
use serde_json::Value;
use serde_json::json;
use std::env;
mod exec;

pub fn main() {
    let window_length: i32 = 500;
    let window_height: i32 = 500;
    let mut done_button = raylib_interactive::button::Button::new(250.0, 250.0, 100.0, 50.0, "done");
    done_button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::WHITE);
    done_button.set_font_size(20);
    let mut screen_width_field = raylib_interactive::textfield::TextField::new(250.0, 100.0, 100.0, 50.0, 500);
    screen_width_field.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY);
    screen_width_field.set_font_size(20);
    let mut screen_height_field = raylib_interactive::textfield::TextField::new(250.0, 200.0, 100.0, 50.0, 500);
    screen_height_field.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY);
    screen_height_field.set_font_size(20);
    let (mut rl, thread) = raylib::init()
        .size(window_length, window_height)
        .title("settings")
        .build();
    while !rl.window_should_close() {
        done_button.update(&mut rl);
        screen_width_field.update(&mut rl);
        screen_height_field.update(&mut rl);

        if done_button.is_clicked(&mut rl) {
            let window_length: i32 = screen_width_field.get_text().parse().unwrap();
            let window_height: i32 = screen_height_field.get_text().parse().unwrap();
            env::set_var("RSWINDOW_LENGTH", window_length.to_string());
            env::set_var("RSWINDOW_HEIGHT", window_height.to_string());
            break;
        }

        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        done_button.draw(&mut d);
        d.draw_text("Screen Width:", 100, 115, 20, Color::BLACK);
        screen_width_field.draw(&mut d);
        d.draw_text("Screen Height:", 100, 215, 20, Color::BLACK);
        screen_height_field.draw(&mut d);
    }
}