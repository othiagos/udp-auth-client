use std::{io::Error, net::UdpSocket};
use super::package::gas::{GASPackageRequest, GASPackageResponse, GASPackageStatus, GASPackageValidation};

const SAS_SIZE_MULTIPLIER: usize = 80;
const BASE_BUFFER_SIZE_REQUEST: usize = 68;
const BASE_BUFFER_SIZE_STATUS: usize = 69;
const ERROR_MSG_ARGUMENTS: &str = "Insufficient arguments provided! Expected more.";
const ERROR_MSG_SEND_PACKAGE: &str = "Failed to send package!";
const ERROR_MSG_RECV_PACKAGE: &str = "Failed to receive package!";
const MAX_RESPONSE_ATTEMPTS: usize = 3;

fn attempt_request<F, G>(socket: &UdpSocket, args: &[String], req_fn: F, res_fn: G)
where
    F: Fn(&UdpSocket, &[String]) -> usize,
    G: Fn(&UdpSocket, usize) -> Result<usize, Error>, 
{
    let mut request_result: Result<usize, Error> = Ok(0);

    for _ in 0..MAX_RESPONSE_ATTEMPTS {
        let sas_len = req_fn(socket, args);
        request_result = res_fn(socket, sas_len);

        if request_result.is_ok() {
            break;
        }
    }

    if let Err(e) = request_result {
        eprintln!("{ERROR_MSG_RECV_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }
}

pub fn gtr(socket: &UdpSocket, args: &[String]) {
    attempt_request(socket, args, request, response);
}

pub fn gtv(socket: &UdpSocket, args: &[String]) {
    attempt_request(socket, args, validation, status);
}

fn make_sas_from_arg(arg: &str) -> Vec<&str> {
    arg.split(":").collect()
}

fn request(socket: &UdpSocket, args: &[String]) -> usize {
    let len = args.first().expect(ERROR_MSG_ARGUMENTS).parse::<usize>().unwrap();

    let vec_sas: Vec<Vec<&str>> = args[1..].iter().map(|sas| make_sas_from_arg(sas)).collect();

    if vec_sas.len() != len {
        eprintln!("Expected {} SAS values, but received {}", len, vec_sas.len());
        std::process::exit(1);
    }

    let pack = GASPackageRequest::new(vec_sas);

    if let Err(e) = socket.send(pack.as_bytes()) {
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }

    len
}

fn response(socket: &UdpSocket, sas_len: usize) -> Result<usize, Error> {
    let buf_len = SAS_SIZE_MULTIPLIER * sas_len + BASE_BUFFER_SIZE_REQUEST;
    let mut buf = vec![0; buf_len];
    buf.resize(buf_len, 0);

    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => return Err(e),
    };

    let pack = GASPackageResponse::new(buf, sas_len);
    pack.print_gas();

    Ok(buf_len)
}

fn validation(socket: &UdpSocket, args: &[String]) -> usize {
    if args.is_empty() {
        eprintln!("{}", ERROR_MSG_ARGUMENTS);
        std::process::exit(1);
    }

    let sas_values: Vec<&str> = args.first().unwrap().split("+").collect();
    let pack = GASPackageValidation::new(&sas_values);

    if let Err(e) = socket.send(pack.as_bytes()) {
        eprintln!("{ERROR_MSG_SEND_PACKAGE} {:?}", e.to_string());
        std::process::exit(1);
    }

    sas_values.len() - 1
}

fn status(socket: &UdpSocket, sas_len: usize) -> Result<usize, Error> {
    let buf_len = SAS_SIZE_MULTIPLIER * sas_len + BASE_BUFFER_SIZE_STATUS;
    let mut buf = vec![0; buf_len];

    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => return Err(e),
    };

    let pack = GASPackageStatus::new(buf, sas_len);
    pack.print_status();

    Ok(buf_len)
}
