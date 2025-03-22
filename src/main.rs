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
        panic!("Insufficient arguments! Expected at least {} arguments, but got {}.", EXPECTED_ARGUMENTS, args.len());
    }

    let server_address = args.get(1).expect("Missing server address").as_ref();
    let port = args.get(2).expect("Missing port").parse().expect("Invalid port number");
    let command = args.get(3).expect("Missing command");

    let socket = UdpSocket::bind(SOCKET_BIND_ADDRESS).expect("Failed to bind socket to address");
    let timeout_duration = Duration::new(TIMEOUT_SECONDS, 0);
    
    socket.set_read_timeout(Some(timeout_duration)).expect("Failed to set read timeout");
    socket.set_write_timeout(Some(timeout_duration)).expect("Failed to set write timeout");

    socket
        .connect((server_address, port))
        .expect("Failed to connect to the server");

    match command.as_str() {
        "itr" => authentication::sas::itr(&socket, &args[EXPECTED_ARGUMENTS..]),
        "itv" => authentication::sas::itv(&socket, &args[EXPECTED_ARGUMENTS..]),
        "gtr" => authentication::gas::gtr(&socket, &args[EXPECTED_ARGUMENTS..]),
        "gtv" => authentication::gas::gtv(&socket, &args[EXPECTED_ARGUMENTS..]),
        _ => panic!("Unknown command: {}", command),
    }
}
