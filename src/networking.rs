use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;
use std::time::Duration;

pub type ClientHandler = Arc<dyn Fn(TcpStream) + Send + Sync + 'static>;

pub struct AsyncTcpServer {
    address: String,
    handler: ClientHandler,
}

impl AsyncTcpServer {
    /// Creates a new AsyncTcpServer instance.
    pub fn new(address: &str, handler: ClientHandler) -> Self {
        Self {
            address: address.to_string(),
            handler,
        }
    }

    /// Starts the TCP server and listens for incoming connections.
    pub async fn run(&self) -> async_std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Server listening on {}", self.address);

        while let Some(stream) = listener.incoming().next().await {
            match stream {
                Ok(stream) => {
                    let handler = Arc::clone(&self.handler);
                    task::spawn(async move {
                        handler(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Starts the TCP server with bidirectional communication.
    pub async fn run_with_messages<F, Fut>(&self, message_handler: F) -> async_std::io::Result<()>
    where
        F: Fn(String, TcpStream) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = async_std::io::Result<()>> + Send + 'static,
    {
        let listener = TcpListener::bind(&self.address).await?;
        println!("Server listening on {}", self.address);

        let message_handler = Arc::new(message_handler); // Wrap in Arc once

        while let Some(stream) = listener.incoming().next().await {
            match stream {
                Ok(stream) => {
                    let handler_clone = Arc::clone(&message_handler); // Clone Arc for this iteration

                    task::spawn(async move {
                        let mut buffer = [0; 1024];
                        let mut stream = stream; // Make stream mutable

                        loop {
                            match stream.read(&mut buffer).await {
                                Ok(0) => break, // Connection closed.
                                Ok(n) => {
                                    let received = String::from_utf8_lossy(&buffer[..n]).to_string();
                                    println!("Server received: {}", received);

                                    // Call the asynchronous message handler.
                                    if let Err(e) = handler_clone(received, stream.clone()).await {
                                        eprintln!("Error handling message: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read from client: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                }
                Err(e) => eprintln!("Failed to accept connection: {}", e),
            }
        }

        Ok(())
    }
}

pub struct AsyncTcpClient {
    address: String,
}

impl AsyncTcpClient {
    /// Connects to a TCP server and returns a TcpStream.
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    pub async fn connect(&self) -> async_std::io::Result<TcpStream> {
        let stream = TcpStream::connect(&self.address).await?;
        println!("Connected to server at {}", self.address);
        Ok(stream)
    }

    /// Sends a message over the given TcpStream.
    pub async fn send(stream: &mut TcpStream, message: &str) -> async_std::io::Result<()> {
        stream.write_all(message.as_bytes()).await?;
        stream.flush().await
    }

    /// Receives a message from the given TcpStream.
    pub async fn receive(stream: &mut TcpStream) -> async_std::io::Result<String> {
        let mut buffer = vec![0u8; 1024];
        let n = stream.read(&mut buffer).await?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }
}
//example tests/usages
#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_basic_server_client_connection() -> async_std::io::Result<()> {
        // Setup server
        let server = AsyncTcpServer::new("127.0.0.1:8080", Arc::new(|_stream| {
            println!("Client connected!");
        }));

        // Start server in background
        task::spawn(async move {
            server.run().await.unwrap();
        });

        // Give server time to start
        task::sleep(Duration::from_millis(100)).await;

        // Connect client
        let client = AsyncTcpClient::new("127.0.0.1:8080");
        let result = client.connect().await;
        assert!(result.is_ok());

        Ok(())
    }

    #[async_std::test]
    async fn test_bidirectional_communication() -> async_std::io::Result<()> {
        // Setup server with message handler
        let server = AsyncTcpServer::new("127.0.0.1:8081", Arc::new(|_stream| {}));

        // Start server with message handling
        task::spawn(async move {
            server.run_with_messages(|msg, mut stream| async move {
                // Echo the message back
                AsyncTcpClient::send(&mut stream, &format!("Echo: {}", msg)).await?;
                Ok(())
            }).await.unwrap();
        });

        task::sleep(Duration::from_millis(100)).await;

        // Connect client
        let client = AsyncTcpClient::new("127.0.0.1:8081");
        let mut stream = client.connect().await?;

        // Send message
        let test_message = "Hello, Server!";
        AsyncTcpClient::send(&mut stream, test_message).await?;

        // Receive response
        let response = AsyncTcpClient::receive(&mut stream).await?;
        assert_eq!(response, format!("Echo: {}", test_message));

        Ok(())
    }

    #[async_std::test]
    async fn test_multiple_clients() -> async_std::io::Result<()> {
        let server = AsyncTcpServer::new("127.0.0.1:8082", Arc::new(|_stream| {}));
        
        task::spawn(async move {
            server.run().await.unwrap();
        });

        task::sleep(Duration::from_millis(100)).await;

        // Connect multiple clients
        let client1 = AsyncTcpClient::new("127.0.0.1:8082");
        let client2 = AsyncTcpClient::new("127.0.0.1:8082");
        let client3 = AsyncTcpClient::new("127.0.0.1:8082");

        let result1 = client1.connect().await;
        let result2 = client2.connect().await;
        let result3 = client3.connect().await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        Ok(())
    }
}
