use raylib::prelude::*;
use raylib_interactive;
use std::env;
use serde_json::json;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::fs;

fn get_settings_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("data.json")
}

fn ensure_settings_exists() {
    let path = get_settings_path();
    if (!path.exists()) {
        let default_settings = json!({
            "settings": {
                "RSWINDOW_LENGTH": "1000",
                "RSWINDOW_HEIGHT": "1000",
                "FPS": "60",
                "PORT": "5766",
                "NAME": "Player",
                "IP": "127.0.0.1",
                "PREFERRED_LATENCY": "40"
            }
        });
        fs::write(&path, serde_json::to_string_pretty(&default_settings).unwrap())
            .expect("Failed to create default settings file");
    }
}

fn read_settings() -> Value {
    let path = get_settings_path();
    
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content)
                .unwrap_or_else(|_| json!({"settings": {}})),
            Err(_) => json!({"settings": {}})
        }
    } else {
        json!({"settings": {}})
    }
}

fn write_settings(settings: &Value) -> Result<(), std::io::Error> {
    let path = get_settings_path();
    let mut data = read_settings();
    data["settings"] = settings.clone();
    
    fs::write(&path, serde_json::to_string_pretty(&data)?)
}

fn token_good(token: &str) -> bool {
    token == "232445" //will change later
}

fn redeem_skin(token: &str) -> Result<(), String> {
    if token.len() == 6 && token_good(token) {
        println!("Skin redeemed successfully with token: {}", token);
        Ok(())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn main() {
    ensure_settings_exists();
    let window_length: i32 = 1000;
    let window_height: i32 = 700;
    let mut done_button = raylib_interactive::button::Button::new(250.0, 500.0, 100.0, 50.0, "done");
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
    let mut more_button = raylib_interactive::button::Button::new(10.0, 10.0, 100.0, 50.0, "More");
    more_button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::WHITE);
    more_button.set_font_size(20);
    let mut token_field = raylib_interactive::textfield::TextField::new(250.0, 50.0, 200.0, 30.0, 6);
    token_field.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY);
    token_field.set_font_size(20);
    let mut redeem_button = raylib_interactive::button::Button::new(460.0, 50.0, 100.0, 30.0, "Redeem");
    redeem_button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::WHITE);
    redeem_button.set_font_size(20);
    let settings = read_settings();
    let mut skin_id: i32 = settings["settings"]["SKIN"]
        .as_str()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let mut skin_field = raylib_interactive::textfield::TextField::new(250.0, 450.0, 100.0, 30.0, 1);
    skin_field.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY);
    skin_field.set_font_size(20);
    if skin_id == 0 {
        skin_field.set_value("0");
    }
    
    // Set colors for all fields
    for field in [&mut screen_width_field, &mut screen_height_field, &mut fps_field, 
                  &mut port_field, &mut name_field, &mut ip_field, &mut latency_field, &mut skin_field].iter_mut() {
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
    skin_field.set_value(&skin_id.to_string());

    fullscreen_button.update(&mut rl);
    while !rl.window_should_close() {
        if skin_id == 0 {
            skin_field.set_value("0");
        }
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
        skin_field.update(&mut rl);
        more_button.update(&mut rl);
        if more_button.is_clicked(&mut rl) {
            // Open skin redemption page
            while !rl.window_should_close() {
                token_field.update(&mut rl);
                redeem_button.update(&mut rl);
                if redeem_button.is_clicked(&mut rl) {
                    let token = token_field.get_text();
                    match redeem_skin(&token) {
                        Ok(_) => {
                            println!("Skin redeemed successfully");
                            skin_id = 1;
                            skin_field.set_value(&skin_id.to_string());
                        },
                        Err(e) => println!("Failed to redeem skin: {}", e)
                    }
                    break;
                }

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::WHITE);
                d.draw_text("Enter 6-digit token:", 100, 55, 20, Color::BLACK);
                token_field.draw(&mut d);
                redeem_button.draw(&mut d);
            }
        }

        if done_button.is_clicked(&mut rl) {
            let settings = json!({
                "RSWINDOW_LENGTH": screen_width_field.get_text(),
                "RSWINDOW_HEIGHT": screen_height_field.get_text(),
                "FPS": fps_field.get_text(),
                "PORT": port_field.get_text(),
                "SKIN": skin_field.get_text(),
                "IP": ip_field.get_text(),
                "PREFERRED_LATENCY": latency_field.get_text(),
            });

            match write_settings(&settings) {
                Ok(_) => println!("Settings saved successfully"),
                Err(e) => println!("Failed to save settings: {}", e)
            }
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
        d.draw_text("Skin:", 100, 455, 20, Color::BLACK);
        skin_field.draw(&mut d);
        more_button.draw(&mut d);
    }
}