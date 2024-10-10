use event_loop_server::servers::tcp_server::start_tcp_server;
use event_loop_server::servers::udp_server::start_udp_server;
use tokio::task;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let tcp_server_task = task::spawn(start_tcp_server("127.0.0.1:8080"));
    let udp_server_task = task::spawn(start_udp_server("127.0.0.1:8081"));

    println!("Servers are running...");

    let _res = tokio::try_join!(tcp_server_task, udp_server_task)?;

    Ok(())
}
