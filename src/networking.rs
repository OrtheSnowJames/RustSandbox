use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;
use std::time::Duration;
use std::os::unix::io::AsRawFd; // For Unix-based systems
#[cfg(windows)]
use std::os::windows::io::AsRawSocket; // For Windows
use std::collections::HashMap;
use std::sync::{Mutex};

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

    /// Gets the socket ID of a TcpStream.
    pub fn get_socket_id(stream: &TcpStream) -> usize {
        #[cfg(unix)]
        {
            stream.as_raw_fd() as usize
        }
        #[cfg(windows)]
        {
            stream.as_raw_socket() as usize
        }
    }

    /// Sends a message to a specific client identified by socket ID
    pub async fn send_to_socket(stream: &mut TcpStream, message: &str, target_socket_id: usize) -> async_std::io::Result<()> {
        if Self::get_socket_id(stream) == target_socket_id {
            stream.write_all(message.as_bytes()).await?;
            stream.flush().await
        } else {
            Ok(())
        }
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

    /// Gets the socket ID of a TcpStream.
    pub fn get_socket_id(stream: &TcpStream) -> usize {
        #[cfg(unix)]
        {
            stream.as_raw_fd() as usize
        }
        #[cfg(windows)]
        {
            stream.as_raw_socket() as usize
        }
    }

    /// Starts a continuous message handling loop
    pub async fn handle_messages<F, Fut>(&self, stream: &mut TcpStream, message_handler: F) -> async_std::io::Result<()>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = async_std::io::Result<()>> + Send + 'static,
    {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    let received = String::from_utf8_lossy(&buffer[..n]).to_string();
                    message_handler(received).await?;
                }
                Err(e) => {
                    eprintln!("Failed to read from server: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    /// Starts an interactive session with the server
    pub async fn start_interactive_session(mut stream: TcpStream) -> async_std::io::Result<()> {
        let mut read_stream = stream.clone();
        
        // Spawn a task to handle incoming messages
        task::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match read_stream.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received: {}", msg);
                    }
                    Err(e) => {
                        eprintln!("Error reading: {}", e);
                        break;
                    }
                }
            }
        });

        // Main task handles sending messages
        let mut input = String::new();
        loop {
            input.clear();
            if std::io::stdin().read_line(&mut input).is_err() {
                break;
            }
            if input.trim() == "quit" {
                break;
            }
            if let Err(e) = Self::send(&mut stream, &input).await {
                eprintln!("Error sending: {}", e);
                break;
            }
        }

        Ok(())
    }
}

// Add this new struct
pub struct ClientConnections {
    connections: HashMap<u32, TcpStream>,
}

impl ClientConnections {
    pub fn new() -> Self {
        ClientConnections {
            connections: HashMap::new()
        }
    }

    pub fn add_client(&mut self, id: u32, stream: TcpStream) {
        self.connections.insert(id, stream);
    }

    pub fn get_client(&mut self, id: u32) -> Option<&mut TcpStream> {
        self.connections.get_mut(&id)
    }
}

//basic other functions
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::Command;

#[cfg(target_os = "windows")]
fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    // Windows implementation using `ipconfig`
    let output = Command::new("ipconfig")
        .arg("/all")
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute `ipconfig` command".into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    // Extract IPv4 address (example: adjust based on your `ipconfig` output)
    if let Some(line) = stdout.lines().find(|l| l.contains("IPv4 Address")) {
        let parts: Vec<&str> = line.split(": ").collect();
        if let Ok(ip) = parts[1].trim().parse::<Ipv4Addr>() {
            return Ok(IpAddr::V4(ip));
        }
    }

    Err("Failed to parse IP address from `ipconfig` output".into())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    // Linux/macOS implementation using `ip`
    let output = Command::new("ip addr")
        .arg("addr")
        .arg("show")
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute `ip` command".into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    // Extract IPv4 address (example: adjust based on your `ip` output)
    if let Some(line) = stdout.lines().find(|l| l.contains("inet ")) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Ok(ip) = parts[1].parse::<Ipv4Addr>() {
            return Ok(IpAddr::V4(ip));
        }
    }

    Err("Failed to parse IP address from `ip` output".into())
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
            server.run().await.expect("Server failed to run");
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
            }).await.expect("Server failed to run with messages");
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
            server.run().await.expect("Server failed to run");
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

    #[async_std::test]
    async fn test_get_socket_id() -> async_std::io::Result<()> {
        // Setup server
        let server = AsyncTcpServer::new("127.0.0.1:8083", Arc::new(|_stream| {}));

        // Start server in background
        task::spawn(async move {
            server.run().await.unwrap();
        });

        // Give server time to start
        task::sleep(Duration::from_millis(100)).await;

        // Connect client
        let client = AsyncTcpClient::new("127.0.0.1:8083");
        let stream = client.connect().await?;

        // Get socket ID
        let socket_id = AsyncTcpClient::get_socket_id(&stream);
        println!("Socket ID: {}", socket_id);

        assert!(socket_id > 0);

        Ok(())
    }

    #[async_std::test]
    async fn test_continuous_messaging() -> async_std::io::Result<()> {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        // Setup server
        let server = AsyncTcpServer::new("127.0.0.1:8084", Arc::new(|_stream| {}));
        let message_count = Arc::new(AtomicUsize::new(0));
        let message_count_clone = message_count.clone();

        // Start server with continuous message handling
        task::spawn(async move {
            server.run_with_messages(move |msg, mut stream| {
                let message_count_clone = Arc::clone(&message_count_clone);
                async move {
                    message_count_clone.fetch_add(1, Ordering::SeqCst);
                    AsyncTcpServer::send(&mut stream, &format!("Echo: {}", msg)).await
                }
            })
            .await
            .expect("Server failed to run");
        });

        task::sleep(Duration::from_millis(100)).await;

        // Connect client
        let client = AsyncTcpClient::new("127.0.0.1:8084");
        let mut stream = client.connect().await?;

        // Send multiple messages
        for i in 0..5 {
            AsyncTcpClient::send(&mut stream, &format!("Message {}", i)).await?;
            let response = AsyncTcpClient::receive(&mut stream).await?;
            assert_eq!(response, format!("Echo: Message {}", i));
        }

        assert_eq!(message_count.load(Ordering::SeqCst), 5);
        Ok(())
    }
}
