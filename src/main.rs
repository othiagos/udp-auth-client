mod authentication;

use std::env;
use std::net::UdpSocket;
use std::time::Duration;

const EXPECTED_ARGUMENTS: usize = 4;
const SOCKET_BIND_ADDRESS: &str = "[::]:0";
const TIMEOUT_SECONDS: u64 = 5;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < EXPECTED_ARGUMENTS {
        eprintln!("Insufficient arguments! Expected at least {} arguments, but got {}.", EXPECTED_ARGUMENTS, args.len());
        std::process::exit(1);
    }

    let server_address = args.get(1).unwrap().as_ref();
    let port = args.get(2).unwrap();
    let command = args.get(3).unwrap();

    let port = match port.parse::<u16>() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Invalid port number: {:?}", e.to_string());
            std::process::exit(1);
        }
    };

    let socket = UdpSocket::bind(SOCKET_BIND_ADDRESS).expect("Failed to bind socket to address");
    let timeout_duration = Duration::new(TIMEOUT_SECONDS, 0);
    
    socket.set_read_timeout(Some(timeout_duration)).expect("Failed to set read timeout");
    socket.set_write_timeout(Some(timeout_duration)).expect("Failed to set write timeout");

    if let Err(e) = socket.connect((server_address, port)) {
        eprintln!("Failed to connect to the server: {:?}", e.to_string());
        std::process::exit(1);
    }

    match command.as_str() {
        "itr" => authentication::sas::itr(&socket, &args[EXPECTED_ARGUMENTS..]),
        "itv" => authentication::sas::itv(&socket, &args[EXPECTED_ARGUMENTS..]),
        "gtr" => authentication::gas::gtr(&socket, &args[EXPECTED_ARGUMENTS..]),
        "gtv" => authentication::gas::gtv(&socket, &args[EXPECTED_ARGUMENTS..]),
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
