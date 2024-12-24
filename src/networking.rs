use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;

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

pub struct AsyncTcpClient;

impl AsyncTcpClient {
    /// Connects to a TCP server and returns a TcpStream.
    pub async fn connect(address: &str) -> async_std::io::Result<TcpStream> {
        let stream = TcpStream::connect(address).await?;
        println!("Connected to server at {}", address);
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

// Example Usage
#[cfg(test)]
mod tests {
    use super::*;
    use async_std::sync::Mutex;
    use std::time::Duration;

    #[async_std::test]
    async fn test_server_client() {
        let received_data = Arc::new(Mutex::new(String::new()));

        // Create a server that echoes received data back
        let handler = {
            let received_data = Arc::clone(&received_data);
            Arc::new(move |stream: TcpStream| {
                let received_data = Arc::clone(&received_data);
                task::spawn(async move {
                    let mut buffer = [0u8; 1024];
                    let mut stream = stream; // Make stream mutable
                    if let Ok(n) = stream.read(&mut buffer).await {
                        let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                        {
                            let mut received_data = received_data.lock().await;
                            *received_data = message.clone();
                        }
                        let _ = stream.write_all(message.as_bytes()).await;
                    }
                });
            })
        };

        let server = Arc::new(AsyncTcpServer::new("127.0.0.1:8080", handler));
        task::spawn({
            let server = Arc::clone(&server);
            async move {
                server
                    .run_with_messages(|received, mut stream| async move {
                        println!("Processing message: {}", received);
                        let response = format!("Server received: {}", received);
                        stream.write_all(response.as_bytes()).await?;
                        stream.flush().await?;
                        Ok(())
                    })
                    .await
                    .unwrap();
            }
        });

        // Give the server a moment to start
        task::sleep(Duration::from_millis(100)).await;

        // Create a client and communicate with the server
        let mut client = AsyncTcpClient::connect("127.0.0.1:8080").await.unwrap();
        AsyncTcpClient::send(&mut client, "Hello, Server!").await.unwrap();
        let response = AsyncTcpClient::receive(&mut client).await.unwrap();

        assert_eq!(response, "Server received: Hello, Server!");
        assert_eq!(*received_data.lock().await, "Hello, Server!");
    }
}
