use rust_sandbox_lib::movement;
use raylib::prelude::*;
use raylib_interactive;
use serde_json::Value;
use serde_json::json;
use std::thread;
use std::env;
mod client;
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

    println!("Starting client...");
    let client_thread: thread::JoinHandle<()> = thread::spawn(|| {
        client::main();
    });
    println!("Client started.");
    client_thread.join().expect("Client thread panicked");
    //add server when networking proplem is solved
}