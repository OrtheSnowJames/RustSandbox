use rust_sandbox_lib::movement;
use raylib::prelude::*;
use raylib_interactive::*;
use serde_json::Value;
use serde_json::json;
use std::thread;
use std::env;
use std::io::{self, Write};
use native_dialog::FileDialog;

mod client;
mod server;
mod randommods;
mod settings;
mod collision;
mod networking;
mod handle_read;

fn main() {
    println!("Starting settings...");
    let settings_thread: thread::JoinHandle<()> = thread::spawn(|| {
        settings::main();
    });
    println!("Settings started.");
    settings_thread.join().expect("Settings thread panicked");

    let args: Vec<String> = env::args().collect();
    let launchfile_content = match std::fs::read_to_string("./launchfile.json") {
        Ok(content) => content,
        Err(_) => {
            println!("launchfile.json not found. Generating a new one...");

            let (mut rl, thread) = raylib::init()
                .size(800, 600)
                .title("Launch Options")
                .build();

            let mut server_checkbox = checkbox::Checkbox::new(350.0, 200.0, 20.0, "Launch Server");
            let mut client_checkbox = checkbox::Checkbox::new(350.0, 250.0, 20.0, "Launch Client");

            while !rl.window_should_close() {
                if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
                    break;
                }

                server_checkbox.update(&rl);
                client_checkbox.update(&rl);

                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::WHITE);
                d.draw_text("Press enter to save changes; cannot be changed later", 100, 100, 20, Color::BLACK);
                server_checkbox.draw(&mut d);
                client_checkbox.draw(&mut d);
            }

            let mut launch_options = vec![];

            if server_checkbox.is_checked() {
                launch_options.push("server.rs".to_string());
            }

            if client_checkbox.is_checked() {
                launch_options.push("client.rs".to_string());
            }

            let launchfile = json!({ "launch": launch_options });
            let launchfile_content = serde_json::to_string_pretty(&launchfile)
                .expect("Failed to serialize launchfile.json");

            std::fs::write("./launchfile.json", &launchfile_content)
                .expect("Failed to write launchfile.json");

            launchfile_content
        }
    };

    let launchfile: Value = serde_json::from_str(&launchfile_content)
        .expect("Failed to parse launchfile.json");

    let launch: Vec<String> = launchfile["launch"]
        .as_array()
        .expect("Expected an array in launchfile.json")
        .iter()
        .map(|v| v.as_str().expect("Expected a string in launchfile.json").to_string())
        .collect();

    if launch.contains(&"server.rs".to_string()) {
        println!("Starting server...");
        let server_thread: thread::JoinHandle<()> = thread::spawn(|| {
            server::main();
        });
    }

    if launch.contains(&"client.rs".to_string()) {
        //2 second timer for client so it doesn't start before server
        println!("Will start client in 2 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        println!("Starting client...");
        let client_thread: thread::JoinHandle<()> = thread::spawn(|| {
            client::main();
        });
        client_thread.join().expect("Client thread panicked");
    }

    println!("Exiting...");
}
