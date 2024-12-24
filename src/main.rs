use raylib::prelude::*;
use raylib_interactive;
use serde_json::Value;
use serde_json::json;
use std::env;
use std::thread;
mod movement;
mod collision;
mod client;
mod settings;

fn main() {
    println!("Starting settings...");
    let settings_thread: thread::JoinHandle<()> = thread::spawn(|| {
        settings::main();
        println!("Settings started.");
    });
    settings_thread.join().expect("Settings thread panicked");

    println!("Starting client...");
    let client_thread: thread::JoinHandle<()> = thread::spawn(|| {
        client::main();
        println!("Client started.");
    });
    client_thread.join().expect("Client thread panicked");
}