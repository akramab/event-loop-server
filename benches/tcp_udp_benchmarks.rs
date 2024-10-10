use criterion::{criterion_group, criterion_main, Criterion};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration, Instant};

// Define the benchmark function for TCP with connection overhead.
async fn benchmark_event_tcp(addr: &str, message: &[u8]) -> tokio::io::Result<Duration> {
    let start = Instant::now();

    // Open a new TCP connection for each request.
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            // Send the message to the TCP server.
            if let Err(e) = stream.write_all(message).await {
                eprintln!("Failed to send message to TCP server: {}", e);
                return Err(e);
            }

            // Prepare a buffer for the response.
            let mut buffer = vec![0; message.len()];

            // Read the echoed response from the server.
            match stream.read_exact(&mut buffer).await {
                Ok(_) => Ok(start.elapsed()),
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

// Define the benchmark function for UDP with a persistent connection.
async fn benchmark_event_udp(socket: &UdpSocket, message: &[u8]) -> tokio::io::Result<Duration> {
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
        Ok(_) => Ok(start.elapsed()),
        Err(e) => {
            eprintln!("Failed to receive response from UDP server: {}", e);
            Err(e)
        }
    }
}

// Criterion benchmark for TCP with delay.
fn benchmark_tcp(c: &mut Criterion) {
    let mut group = c.benchmark_group("TCP Round-trip Time with Connection Overhead");
    let rt = Runtime::new().unwrap();
    let tcp_addr = "127.0.0.1:8080";
    let message = b"ping";

    group.bench_function("tcp_round_trip", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Add a slight delay to simulate real-world traffic and avoid port exhaustion.
                sleep(Duration::from_millis(10)).await;
                let duration = benchmark_event_tcp(tcp_addr, message).await.unwrap();
                // println!("TCP round-trip time: {}µs", duration.as_micros());
            });
        });
    });

    group.finish();
}

// Criterion benchmark for UDP with a single persistent socket.
fn benchmark_udp(c: &mut Criterion) {
    let mut group = c.benchmark_group("UDP Round-trip Time with Persistent Connection");
    let rt = Runtime::new().unwrap();
    let udp_addr = "127.0.0.1:8081";
    let bind_addr = "127.0.0.1:0";
    let message = b"ping";

    // Create a single persistent UDP socket.
    let socket = rt.block_on(async {
        let socket = UdpSocket::bind(bind_addr)
            .await
            .expect("Failed to bind UDP socket");
        socket
            .connect(udp_addr)
            .await
            .expect("Failed to connect UDP socket");
        socket
    });

    group.bench_function("udp_round_trip", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Add a slight to be fair with TCP added delay
                sleep(Duration::from_millis(10)).await;
                let duration = benchmark_event_udp(&socket, message).await.unwrap();
                // println!("UDP round-trip time: {}µs", duration.as_micros());
            });
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_tcp, benchmark_udp);
criterion_main!(benches);
