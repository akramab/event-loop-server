use crate::event_handler::handle_event;
use crate::events::parse_event;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub async fn start_udp_server(addr: &str) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind(addr).await?;
    // 32 is an arbitrary number choosen
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a separate task to handle events.
    let _event_loop = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            handle_event(event).await;
        }
    });

    println!("UDP server listening on {}", addr);

    let mut buffer = [0; 1024];

    loop {
        // Receive a message and get the data along with the sender's address.
        match socket.recv_from(&mut buffer).await {
            Ok((n, addr)) => {
                let data = buffer[..n].to_vec();
                let event = parse_event(data.clone());

                // Send the event to the event handler.
                if let Err(e) = tx.send(event).await {
                    eprintln!("Failed to send event: {}", e);
                }

                // Log the data received and from where.
                println!(
                    "UDP Server Received data from {}: {:?}",
                    addr,
                    String::from_utf8_lossy(&data)
                );

                // Echo the message back to the sender.
                if let Err(e) = socket.send_to(&data, addr).await {
                    eprintln!("Failed to send response to {}: {}", addr, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to receive message: {}", e);
            }
        }
    }
}
