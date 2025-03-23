use std::net::UdpSocket;

use super::package::sas::{SASPackageRequest, SASPackageResponse, SASPackageStatus, SASPackageValidation};

const MIN_REQUEST_ARGS: usize = 2;
const MIN_VALIDATION_ARGS: usize = 1;
const REQUEST_BUFFER_SIZE: usize = 82;
const STATUS_BUFFER_SIZE: usize = 100;
const EXPECTED_SAS_PARTS: usize = 3;
const ARGUMENT_ERROR: &str = "Insufficient arguments provided!";
const ERROR_MSG_SEND_PACKAGE: &str = "Failed to send package!";
const ERROR_MSG_RECV_PACKAGE: &str = "Failed to receive package!";

pub fn itr(socket: &UdpSocket, args: &[String]) {
    request(socket, args);
    response(socket);
}

pub fn itv(socket: &UdpSocket, args: &[String]) {
    validation(socket, args);
    status(socket);
}

fn request(socket: &UdpSocket, args: &[String]) {
    if args.len() < MIN_REQUEST_ARGS {
        eprintln!("{}", ARGUMENT_ERROR);
        std::process::exit(1);
    }

    let id = args.first().unwrap();
    let nonce = args.get(1).unwrap();
    let pack = SASPackageRequest::new(id, nonce);

    if let Err(e) = socket.send(pack.as_bytes()) {
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {e:?}");
        std::process::exit(1);
    }
}

fn response(socket: &UdpSocket) {
    let mut buf = vec![0; REQUEST_BUFFER_SIZE];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => {
            eprintln!("{} {:?}", ERROR_MSG_RECV_PACKAGE, e.to_string());
            std::process::exit(1);
        }
    };

    let pack = SASPackageResponse::new(buf);
    pack.print_sas();
}

fn validation(socket: &UdpSocket, args: &[String]) {
    if args.len() < MIN_VALIDATION_ARGS {
        eprintln!("{}", ARGUMENT_ERROR);
        std::process::exit(1);
    }

    let sas: Vec<&str> = args.first().unwrap().split(':').collect();

    if sas.len() < EXPECTED_SAS_PARTS {
        eprintln!("{}", ARGUMENT_ERROR);
        std::process::exit(1);
    }

    let id = sas[0];
    let nonce = sas[1];
    let token = sas[2];

    let pack = SASPackageValidation::new(id, nonce, token);

    if let Err(e) = socket.send(pack.as_bytes()) {
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {e:?}");
        std::process::exit(1);
    }
}

fn status(socket: &UdpSocket) {
    let mut buf = vec![0; STATUS_BUFFER_SIZE];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => {
            eprintln!("{} {:?}", ERROR_MSG_RECV_PACKAGE, e.to_string());
            std::process::exit(1);
        }
    };

    let pack = SASPackageStatus::new(buf);
    pack.print_status();
}
