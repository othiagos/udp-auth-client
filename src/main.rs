mod communication;
mod package;

use std::env;
use std::net::UdpSocket;

const NUMBER_ARGUMENTS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < NUMBER_ARGUMENTS {
        panic!("few arguments!");
    }

    let server = args.get(1).unwrap();
    let port = args.get(2).unwrap();
    let command = args.get(3).unwrap();

    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address!");
    socket
        .connect(format!("{server}:{port}"))
        .expect("connect function failed!");

    match command.as_ref() {
        "itr" => communication::itr(&socket, &args[NUMBER_ARGUMENTS..]),
        "itv" => communication::itv(&socket, &args[NUMBER_ARGUMENTS..]),
        "gtr" => communication::gtr(&socket, &args[NUMBER_ARGUMENTS..]),
        "gtv" => communication::gtv(&socket, &args[NUMBER_ARGUMENTS..]),
        _ => panic!("unknown command!"),
    }
}
