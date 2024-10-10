use crate::event_handler::handle_event;
use crate::events::parse_event;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::task;

pub async fn start_tcp_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    // 32 is an arbitrary number choosen
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a separate task to handle events.
    task::spawn(async move {
        while let Some(event) = rx.recv().await {
            handle_event(event).await;
        }
    });

    println!("TCP server listening on {}", addr);

    loop {
        // Accept new incoming TCP connections.
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("New connection from {}", addr);
                let tx = tx.clone();

                // Spawn a new task for handling the connection.
                task::spawn(async move {
                    let mut buffer = [0; 1024];

                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                // Connection was closed by the client.
                                println!("Connection closed by {}", addr);
                                break;
                            }
                            Ok(n) => {
                                // Read data and process the event.
                                let data = buffer[..n].to_vec();
                                let event = parse_event(data.clone());

                                if let Err(e) = tx.send(event).await {
                                    eprintln!("Failed to send event from {}: {}", addr, e);
                                }

                                // Echo the received data back to the client.
                                if let Err(e) = socket.write_all(&data).await {
                                    eprintln!("Failed to write to {}: {}", addr, e);
                                    break;
                                }

                                // Log the message.
                                println!(
                                    "TCP Server Processed data from {}: {:?}",
                                    addr,
                                    String::from_utf8_lossy(&data)
                                );
                            }
                            Err(e) => {
                                // Handle read errors.
                                eprintln!("Failed to read from {}: {}", addr, e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                // Handle errors in accepting new connections.
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
