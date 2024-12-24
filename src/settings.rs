use raylib::prelude::*;
use raylib_interactive;
use std::env;
use serde_json::json;
use serde_json::Value;

pub fn main() {
    let window_length: i32 = 500;
    let window_height: i32 = 500;
    let mut done_button = raylib_interactive::button::Button::new(250.0, 460.0, 100.0, 50.0, "done");
    done_button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::WHITE);
    done_button.set_font_size(20);
    let mut fullscreen_button = raylib_interactive::button::Button::new(250.0, 50.0, 150.0, 30.0, "Use Screen Size");
    fullscreen_button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::WHITE);
    fullscreen_button.set_font_size(10);
    let mut screen_width_field = raylib_interactive::textfield::TextField::new(250.0, 100.0, 100.0, 30.0, 1600);



    let mut screen_height_field = raylib_interactive::textfield::TextField::new(250.0, 150.0, 100.0, 30.0, 1000);
    let mut fps_field = raylib_interactive::textfield::TextField::new(250.0, 200.0, 100.0, 30.0, 60);
    let mut port_field = raylib_interactive::textfield::TextField::new(250.0, 250.0, 100.0, 30.0, 5766);
    let mut name_field = raylib_interactive::textfield::TextField::new(250.0, 300.0, 200.0, 30.0, 32);
    let mut ip_field = raylib_interactive::textfield::TextField::new(250.0, 350.0, 200.0, 30.0, 32);
    let mut latency_field = raylib_interactive::textfield::TextField::new(250.0, 400.0, 100.0, 30.0, 1);
    
    // Set colors for all fields
    for field in [&mut screen_width_field, &mut screen_height_field, &mut fps_field, 
                  &mut port_field, &mut name_field, &mut ip_field, &mut latency_field].iter_mut() {
        field.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY);
        field.set_font_size(20);
    }

    let (mut rl, thread) = raylib::init()
        .size(window_length, window_height)
        .title("Settings")
        .build();
    //fullscreen auto
    let monitor: i32 = 0;
    let monitor_width = rl.get_screen_width();
    let monitor_height = rl.get_screen_height();
    //set fields to environment variables
    screen_width_field.set_value(&env::var("RSWINDOW_LENGTH").unwrap_or_else(|_| "1000".to_string()));
    screen_height_field.set_value(&env::var("RSWINDOW_HEIGHT").unwrap_or_else(|_| "1000".to_string()));
    fps_field.set_value(&env::var("FPS").unwrap_or_else(|_| "60".to_string()));
    port_field.set_value(&env::var("PORT").unwrap_or_else(|_| "5766".to_string()));
    name_field.set_value(&env::var("NAME").unwrap_or_else(|_| "Player".to_string()));
    ip_field.set_value(&env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string()));
    latency_field.set_value(&env::var("PREFERRED_LATENCY").unwrap_or_else(|_| "40".to_string()));

    fullscreen_button.update(&mut rl);
    while !rl.window_should_close() {
    if fullscreen_button.is_clicked(&mut rl) {
        screen_width_field.set_value(&monitor_width.to_string());
        screen_height_field.set_value(&monitor_height.to_string());
    }
        // Update all fields
        done_button.update(&mut rl);
        screen_width_field.update(&mut rl);
        screen_height_field.update(&mut rl);
        fullscreen_button.update(&mut rl);
        fps_field.update(&mut rl);
        port_field.update(&mut rl);
        name_field.update(&mut rl);
        ip_field.update(&mut rl);
        latency_field.update(&mut rl);

        if done_button.is_clicked(&mut rl) {
            // Save all settings to environment variables
            env::set_var("RSWINDOW_LENGTH", screen_width_field.get_text());
            env::set_var("RSWINDOW_HEIGHT", screen_height_field.get_text());
            env::set_var("FPS", fps_field.get_text());
            env::set_var("PORT", port_field.get_text());
            env::set_var("NAME", name_field.get_text());
            env::set_var("IP", ip_field.get_text());
            env::set_var("PREFERRED_LATENCY", latency_field.get_text());
            //construct json array with settings
            let settings = json!({
                "RSWINDOW_LENGTH": screen_width_field.get_text(),
                "RSWINDOW_HEIGHT": screen_height_field.get_text(),
                "FPS": fps_field.get_text(),
                "PORT": port_field.get_text(),
                "NAME": name_field.get_text(),
                "IP": ip_field.get_text(),
                "PREFERRED_LATENCY": latency_field.get_text(),
            });
            //add/overwrite to data.json
            let mut data = json!({});
            if std::path::Path::new("data.json").exists() {
                let file = std::fs::File::open("data.json").unwrap();
                data = serde_json::from_reader(file).unwrap();
            }
            let mut file = std::fs::File::create("data.json").unwrap();
            data["settings"] = settings;
            serde_json::to_writer_pretty(&mut file, &data).unwrap();
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        done_button.draw(&mut d);
        // Draw all labels and fields
        d.draw_text("Screen Width:", 100, 105, 20, Color::BLACK);
        screen_width_field.draw(&mut d);
        d.draw_text("Screen Height:", 100, 155, 20, Color::BLACK);
        screen_height_field.draw(&mut d);
        fullscreen_button.draw(&mut d);
        d.draw_text("FPS:", 100, 205, 20, Color::BLACK);
        fps_field.draw(&mut d);
        d.draw_text("Port:", 100, 255, 20, Color::BLACK);
        port_field.draw(&mut d);
        d.draw_text("Name:", 100, 305, 20, Color::BLACK);
        name_field.draw(&mut d);
        d.draw_text("IP:", 100, 355, 20, Color::BLACK);
        ip_field.draw(&mut d);
        d.draw_text("Preferred Latency:", 30, 405, 20, Color::BLACK);
        latency_field.draw(&mut d);
    }
}