mod authentication;

use std::env;
use std::net::UdpSocket;
use std::time::Duration;

const NUMBER_ARGUMENTS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < NUMBER_ARGUMENTS {
        panic!("few arguments!");
    }

    let server: &str = args.get(1).unwrap().as_ref();
    let port = args.get(2).unwrap().parse::<u16>().unwrap();
    let command = args.get(3).unwrap();

    let socket = UdpSocket::bind("[::]:0").expect("couldn't bind to address!");
    let duration = Duration::new(30, 0);
    socket.set_read_timeout(Some(duration)).expect("msg");
    socket.set_write_timeout(Some(duration)).expect("msg");

    socket
        .connect((server, port))
        .expect("connect to the server failed!");

    match command.as_ref() {
        "itr" => authentication::sas::itr(&socket, &args[NUMBER_ARGUMENTS..]),
        "itv" => authentication::sas::itv(&socket, &args[NUMBER_ARGUMENTS..]),
        "gtr" => authentication::gas::gtr(&socket, &args[NUMBER_ARGUMENTS..]),
        "gtv" => authentication::gas::gtv(&socket, &args[NUMBER_ARGUMENTS..]),
        _ => panic!("unknown command!"),
    }
}
