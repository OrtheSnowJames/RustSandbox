# Async TCP Server and Client in Rust

This documentation covers the usage of the `AsyncTcpServer` and `AsyncTcpClient` implementations for network communication in Rust.

## Server Usage

```rust
use std::sync::Arc;
use async_std::net::TcpStream;

// Create a simple server handler
let handler = Arc::new(|stream: TcpStream| {
    // Handle incoming connection
});

// Initialize the server
let server = AsyncTcpServer::new("127.0.0.1:8080", handler);

// Run the server with basic handling
async_std::task::block_on(async {
    server.run().await.unwrap();
});

// Or run with message handling
async_std::task::block_on(async {
    server.run_with_messages(|received, mut stream| async move {
        // Process received message
        let response = format!("Server received: {}", received);
        stream.write_all(response.as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }).await.unwrap();
});
```

## Client Usage

```rust
use async_std::task;

async_std::task::block_on(async {
    // Connect to server
    let mut client = AsyncTcpClient::connect("127.0.0.1:8080").await.unwrap();
    
    // Send message
    AsyncTcpClient::send(&mut client, "Hello, Server!").await.unwrap();
    
    // Receive response
    let response = AsyncTcpClient::receive(&mut client).await.unwrap();
    println!("Received: {}", response);
});
```

## Features

- Asynchronous TCP server and client implementation
- Support for bidirectional communication
- Customizable message handling
- Built with `async-std`

## Error Handling

Both server and client implementations return `async_std::io::Result<T>`, allowing for proper error handling using the `?` operator or match expressions.