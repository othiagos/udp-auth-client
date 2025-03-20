use std::net::UdpSocket;

use super::package::sas::{SASPackageRequest, SASPackageResponse, SASPackageStatus, SASPackageValidation};

pub fn itr(socket: &UdpSocket, args: &[String]) {
    request(socket, args);
    response(socket);
}

pub fn itv(socket: &UdpSocket, args: &[String]) {
    validation(socket, args);
    status(socket);
}

fn request(socket: &UdpSocket, args: &[String]) {
    if args.len() < 2 {
        panic!("few arguments: expected more arguments!");
    }

    let id = args.first().unwrap();
    let nonce = args.get(1).unwrap();
    let pack = SASPackageRequest::new(id, nonce);

    socket
        .send(pack.as_bytes())
        .expect("couldn't send package!");
}

fn response(socket: &UdpSocket) {
    let mut buf = [0; 100];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let pack = SASPackageResponse::new(buf);
    pack.print_sas();
}

fn validation(socket: &UdpSocket, args: &[String]) {
    if args.is_empty() {
        panic!("few arguments: expected more arguments!");
    }

    let sas: Vec<&str> = args.first().unwrap().split(":").collect();

    let id = *sas.first().unwrap();
    let nonce = *sas.get(1).unwrap();
    let token = *sas.get(2).unwrap();

    let pack = SASPackageValidation::new(id, nonce, token);

    socket
        .send(pack.as_bytes())
        .expect("couldn't send package!");
}

fn status(socket: &UdpSocket) {
    let mut buf = [0; 100];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let pack = SASPackageStatus::new(buf);
    pack.print_status();
}
