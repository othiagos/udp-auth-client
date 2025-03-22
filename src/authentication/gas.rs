use std::net::UdpSocket;
use super::package::gas::{GASPackageRequest, GASPackageResponse, GASPackageStatus, GASPackageValidation};

const SAS_SIZE_MULTIPLIER: usize = 80;
const BASE_BUFFER_SIZE_REQUEST: usize = 68;
const BASE_BUFFER_SIZE_STATUS: usize = 69;
const ERROR_MSG_ARGUMENTS: &str = "Insufficient arguments provided! Expected more.";
const ERROR_MSG_SEND_PACKAGE: &str = "Failed to send package!";
const ERROR_MSG_RECV_PACKAGE: &str = "Failed to receive package!";

pub fn gtr(socket: &UdpSocket, args: &[String]) {
    let sas_len = request(socket, args);
    response(socket, sas_len);
}

pub fn gtv(socket: &UdpSocket, args: &[String]) {
    let sas_len = validation(socket, args);
    status(socket, sas_len);
}

fn make_sas_from_arg(arg: &str) -> Vec<&str> {
    arg.split(":").collect()
}

fn request(socket: &UdpSocket, args: &[String]) -> usize {
    let len = args.first().expect(ERROR_MSG_ARGUMENTS).parse::<usize>().unwrap();

    let vec_sas: Vec<Vec<&str>> = args[1..].iter().map(|sas| make_sas_from_arg(sas)).collect();

    if vec_sas.len() != len {
        panic!("Expected {} SAS values, but received {}", len, vec_sas.len());
    }

    let pack = GASPackageRequest::new(vec_sas);

    socket
        .send(pack.as_bytes())
        .expect(ERROR_MSG_SEND_PACKAGE);

    len
}

fn response(socket: &UdpSocket, sas_len: usize) {
    let buf_len = SAS_SIZE_MULTIPLIER * sas_len + BASE_BUFFER_SIZE_REQUEST;
    let mut buf = vec![0; buf_len];
    buf.resize(buf_len, 0);

    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("{} {:?}", ERROR_MSG_RECV_PACKAGE, e),
    };

    let pack = GASPackageResponse::new(buf, sas_len);
    pack.print_gas();
}

fn validation(socket: &UdpSocket, args: &[String]) -> usize {
    if args.is_empty() {
        panic!("{}", ERROR_MSG_ARGUMENTS);
    }

    let sas_values: Vec<&str> = args.first().unwrap().split("+").collect();
    let pack = GASPackageValidation::new(&sas_values);

    socket
        .send(pack.as_bytes())
        .expect(ERROR_MSG_SEND_PACKAGE);

    sas_values.len() - 1
}

fn status(socket: &UdpSocket, sas_len: usize) {
    let buf_len = SAS_SIZE_MULTIPLIER * sas_len + BASE_BUFFER_SIZE_STATUS;
    let mut buf = vec![0; buf_len];

    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("{} {:?}", ERROR_MSG_RECV_PACKAGE, e),
    };

    let pack = GASPackageStatus::new(buf, sas_len);
    pack.print_status();
}
