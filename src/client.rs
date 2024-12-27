use async_std::path::PathBuf;
use raylib::prelude::*;
use raylib_interactive;
use raylib_interactive::button::Button;
use serde_json::Value;
use serde_json::json;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use std::ops::DerefMut;
use crate::movement;
use crate::collision;
use crate::networking::*;
use crate::randommods::get_socket_id;
use super::*;
use crate::randommods;
use async_std::task;
use std::time::Duration;

pub fn main() {
    // Read settings from data.json
    let settings: Value = if std::path::Path::new("data.json").exists() {
        let file = std::fs::File::open("data.json").unwrap();
        let data: Value = serde_json::from_reader(file).unwrap();
        data["settings"].clone()
    } else {
        json!({
            "RSWINDOW_LENGTH": "1000",
            "RSWINDOW_HEIGHT": "1000",
            "FPS": "60",
            "IP": "127.0.0.1",
            "PORT": "5766",
            "PREFERRED_LATENCY": "4",
            "SKIN": "0"
        })
    };

    let mut window_length: i32 = settings["RSWINDOW_LENGTH"].as_str()
        .unwrap_or("1000")
        .parse()
        .unwrap();
    let mut window_height: i32 = settings["RSWINDOW_HEIGHT"].as_str()
        .unwrap_or("1000")
        .parse()
        .unwrap();

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
    
    let mut client = AsyncTcpClient::new(format!("{}:{}", settings["IP"].as_str().unwrap(), settings["PORT"].as_str().unwrap()).as_str());
    let io_stream = Arc::new(Mutex::new({
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 5;
        const RETRY_DELAY_MS: u64 = 500;

        loop {
            match task::block_on(client.connect()) {
                Ok(stream) => break stream,
                Err(e) => {
                    if retry_count >= MAX_RETRIES {
                        panic!("Failed to connect after {} retries: {}", MAX_RETRIES, e);
                    }
                    println!("Connection attempt {} failed, retrying in {}ms...", retry_count + 1, RETRY_DELAY_MS);
                    task::block_on(async_std::task::sleep(Duration::from_millis(RETRY_DELAY_MS)));
                    retry_count += 1;
                }
            }
        }
    }));
    let sockID: i32 = AsyncTcpClient::get_socket_id(&io_stream.lock().unwrap()) as i32;

    // Create message channel for communication between render and network threads
    let (tx, rx) = async_std::channel::bounded(100);
    let tx_clone = tx.clone();

    // Spawn network receive handler
    let io_stream_clone = Arc::clone(&io_stream);
    task::spawn(async move {
        let mut stream = io_stream_clone.lock().unwrap().deref_mut().clone();
        client.handle_messages(&mut stream, move |msg| {
            let tx = tx_clone.clone();
            async move {
                if let Ok(msg_value) = serde_json::from_str::<Value>(&msg) {
                    tx.send(msg_value).await.unwrap_or_else(|e| eprintln!("Send error: {}", e));
                }
                Ok(())
            }
        }).await.unwrap_or_else(|e| eprintln!("Network error: {}", e));
    });

    let mut movement = movement::Movement {
        position: Vector2::new(400.0, 250.0),
        speed: 5.0,
        width: 50,
        height: 50,
    };

    let mut checklist: Value = json!({
        "x": 400,
        "y": 250,
        "width": 50,
        "height": 50,
        "id": get_socket_id(&*io_stream.lock().unwrap()),
        "initGameFully": false,
        "localPlayerSet": false,
        "room": 1,
        //spritestate uses cardinal directions
        "spriteState": 3,
        "skin": settings["SKIN"].as_str().unwrap_or("0").parse::<i32>().unwrap(),
        "shields": 0,
    });

    //define game here
    let room_in: i32 = 1;
    let whole_room_in: String = "room".to_string() + &room_in.to_string();
    let game: Arc<Mutex<Value>> = Arc::new(Mutex::new(json!({
        "room1": {
            "objects": [
                //square that player moves on
                {"x": 0, "y": 0, "width": 1000, "height": 1000, "id": 0},
            ],
            "players": [],
            "npcs": [],
            "roomID": 1
        },
    })));



    let objects_interpret_inside: Value = json!([
        0
    ]);
    let mut open: bool = rl.window_should_close();


    let mut button = Button::new(((window_length as i32) / 2) as f32, ((window_height as i32) / 2) as f32, 100 as f32, 50 as f32, "position");
    button.set_colors(Color::GRAY, Color::DARKGRAY, Color::LIGHTGRAY, Color::BLACK, Color::BLACK);
    button.set_font_size(10);
    //loop
    let game_clone = Arc::clone(&game);
    let io_stream_clone_for_receive = Arc::clone(&io_stream);
    let receive_thread: thread::JoinHandle<()> = thread::spawn(move || {
        let io_stream = io_stream_clone_for_receive;
        while open {
            let message = task::block_on(AsyncTcpClient::receive(&mut io_stream.lock().unwrap())).unwrap();
            println!("Received: {}", message);
            handle_read::handle_readd::handle_read_msg(&message, Arc::clone(&game_clone), &mut io_stream.lock().unwrap());
        }
    });

    while !rl.window_should_close() {
        button.update(&mut rl);
        if button.is_clicked(&mut rl) {
            movement.position.x = 400.0;
            movement.position.y = 250.0;
        }
        movement.update(rl.get_frame_time());
        {
            let mut game_lock = game.lock().unwrap();
            let player_data = json!({
                "id": "player1",
                "x": movement.position.x,
                "y": movement.position.y
            });
            if let Some(players) = game_lock[whole_room_in.clone()]["players"].as_array_mut() {
                if let Some(player) = players.iter_mut().find(|p| p["id"] == "player1") {
                    *player = player_data;
                } else {
                    players.push(player_data);
                }
            }
        }
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        //get collisions
        if let Some(objects) = game.lock().unwrap()[whole_room_in.clone()]["objects"].as_array() {
            for object in objects {
                if objects_interpret_inside.as_array().unwrap().contains(&object["id"]) {
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

        // Send position updates
        let update_msg = json!({
            "type": "update",
            "player_id": sockID.to_string(),
            "data": {
                "x": movement.position.x,
                "y": movement.position.y
            }
        });

        task::block_on(AsyncTcpClient::send(&mut io_stream.lock().unwrap(), &update_msg.to_string())).unwrap_or_else(|e| eprintln!("Send error: {}", e));

        // Process received messages
        while let Ok(msg) = rx.try_recv() {
            match msg["type"].as_str() {
                Some("update_confirm") => {
                    // Handle confirmation
                },
                Some(msg_type) => println!("Received message type: {}", msg_type),
                None => println!("Received invalid message format")
            }
        }
    }
    //closing sequence
    open = rl.window_should_close();
    receive_thread.join().unwrap();
}