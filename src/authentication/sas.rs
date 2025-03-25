use std::{io::Error, net::UdpSocket};

use super::package::sas::{SASPackageRequest, SASPackageResponse, SASPackageStatus, SASPackageValidation};

const MIN_REQUEST_ARGS: usize = 2;
const MIN_VALIDATION_ARGS: usize = 1;
const REQUEST_BUFFER_SIZE: usize = 82;
const STATUS_BUFFER_SIZE: usize = 100;
const EXPECTED_SAS_PARTS: usize = 3;
const ARGUMENT_ERROR: &str = "Insufficient arguments provided!";
const ERROR_MSG_SEND_PACKAGE: &str = "Failed to send package!";
const ERROR_MSG_RECV_PACKAGE: &str = "Failed to receive package!";
const MAX_RESPONSE_ATTEMPTS: usize = 3;

fn attempt_request<F, G>(socket: &UdpSocket, args: &[String], req_fn: F, res_fn: G)
where
    F: Fn(&UdpSocket, &[String]),
    G: Fn(&UdpSocket) -> Result<usize, Error>,
{
    let mut request_result: Result<usize, Error> = Ok(0);

    for _ in 0..MAX_RESPONSE_ATTEMPTS {
        req_fn(socket, args);
        request_result = res_fn(socket);

        if request_result.is_ok() {
            break;
        }
    }

    if let Err(e) = request_result {
        eprintln!("{ERROR_MSG_RECV_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }
}


pub fn itr(socket: &UdpSocket, args: &[String]) {
    attempt_request(socket, args, request, response);
}

pub fn itv(socket: &UdpSocket, args: &[String]) {
    attempt_request(socket, args, validation, status);
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
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }
}

fn response(socket: &UdpSocket) -> Result<usize, Error> {
    let mut buf = vec![0; REQUEST_BUFFER_SIZE];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => return Err(e)
    };

    let pack = SASPackageResponse::new(buf);
    pack.print_sas();

    Ok(buf.len())
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
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }
}

fn status(socket: &UdpSocket) -> Result<usize, Error> {
    let mut buf = vec![0; STATUS_BUFFER_SIZE];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => return Err(e)
    };

    let pack = SASPackageStatus::new(buf);
    pack.print_status();

    Ok(buf.len())
}
