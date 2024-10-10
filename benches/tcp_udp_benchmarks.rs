use criterion::{criterion_group, criterion_main, Criterion};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Runtime;
use tokio::time::{Duration, Instant};

// Define the benchmark function for TCP.
async fn benchmark_event_tcp(addr: &str, message: &[u8]) -> tokio::io::Result<Duration> {
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            let start = Instant::now();

            // Send the message to the TCP server.
            if let Err(e) = stream.write_all(message).await {
                eprintln!("Failed to send message to TCP server: {}", e);
                return Err(e);
            }

            // Prepare a buffer for the response.
            let mut buffer = vec![0; message.len()];

            // Read the echoed response from the server.
            match stream.read_exact(&mut buffer).await {
                Ok(_) => {
                    let duration = start.elapsed();
                    println!(
                        "Received response from TCP server: {:?}",
                        String::from_utf8_lossy(&buffer)
                    );
                    Ok(duration)
                }
                Err(e) => {
                    eprintln!("Failed to read from TCP server: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to TCP server: {}", e);
            Err(e)
        }
    }
}

// Define the benchmark function for UDP.
async fn benchmark_event_udp(
    addr: &str,
    message: &[u8],
    bind_addr: &str,
) -> tokio::io::Result<Duration> {
    match UdpSocket::bind(bind_addr).await {
        Ok(socket) => {
            // Connect the UDP socket to the server's address.
            if let Err(e) = socket.connect(addr).await {
                eprintln!("Failed to connect UDP socket: {}", e);
                return Err(e);
            }

            let start = Instant::now();

            // Send the message to the UDP server.
            if let Err(e) = socket.send(message).await {
                eprintln!("Failed to send message to UDP server: {}", e);
                return Err(e);
            }

            // Prepare a buffer for the response.
            let mut buffer = vec![0; message.len()];

            // Read the echoed response from the server.
            match socket.recv(&mut buffer).await {
                Ok(_) => {
                    let duration = start.elapsed();
                    println!(
                        "Received response from UDP server: {:?}",
                        String::from_utf8_lossy(&buffer)
                    );
                    Ok(duration)
                }
                Err(e) => {
                    eprintln!("Failed to receive response from UDP server: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to bind UDP socket: {}", e);
            Err(e)
        }
    }
}

// Criterion benchmark for TCP.
fn benchmark_tcp(c: &mut Criterion) {
    let mut group = c.benchmark_group("TCP Round-trip Time");
    let tcp_addr = "127.0.0.1:8080";
    let message = b"ping";

    group.bench_function("tcp_round_trip", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                benchmark_event_tcp(tcp_addr, message).await.unwrap();
            });
        });
    });

    group.finish();
}

// Criterion benchmark for UDP.
fn benchmark_udp(c: &mut Criterion) {
    let mut group = c.benchmark_group("UDP Round-trip Time");
    let udp_addr = "127.0.0.1:8081";
    let bind_addr = "127.0.0.1:0";
    let message = b"ping";

    group.bench_function("udp_round_trip", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                benchmark_event_udp(udp_addr, message, bind_addr)
                    .await
                    .unwrap();
            });
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_tcp, benchmark_udp);
criterion_main!(benches);
