use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(
    name = "chnett",
    version = "1.0",
    about = "Check Ports because your Rad to the bone :<"
)]
struct Args {
    // Target hostname or IP address
    #[arg(long)]
    host: String,

    // Connection timeout in seconds
    #[arg(long, default_value_t = 2)]
    timeout: u64,
}

enum PortState {
    Open(Duration),
    Closed,
    Filtered,
    Error(String),
}

fn main() {
    let args = Args::parse();

    let timeout = Duration::from_secs(args.timeout);

    println!("Checking connectivity to {}", args.host);

    let host = args.host;

    let mut handles = vec![];

    // Spawn all threads
    for port in 1..=1000 {
        let host = host.to_string();

        let handle = thread::spawn(move || (port, check_port(&host, port, timeout)));

        handles.push(handle);
    }

    // Collect and print results in order
    for handle in handles {
        let (port, result) = handle.join().unwrap();

        match result {
            PortState::Open(elapsed) => {
                println!("Port {}: OPEN ({}ms)", port, elapsed.as_millis());
            }
            PortState::Closed => {
                println!("Port {}: CLOSED", port);
            }
            PortState::Filtered => {}
            PortState::Error(error) => {
                println!("Port {}: ERROR ({})", port, error);
            }
        }
    }

    println!("Port scanning complete!");
}

fn check_port(host: &str, port: u16, timeout: Duration) -> PortState {
    let address = format!("{}:{}", host, port);

    let socket_addr = match address.to_socket_addrs() {
        Ok(mut addresses) => match addresses.next() {
            Some(addr) => addr,
            None => return PortState::Error("No addresses found".to_string()),
        },
        Err(e) => return PortState::Error(e.to_string()),
    };

    let start = Instant::now();

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => PortState::Open(start.elapsed()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::ConnectionRefused => PortState::Closed,
            std::io::ErrorKind::TimedOut => PortState::Filtered,
            _ => PortState::Error(e.to_string()),
        },
    }
}
