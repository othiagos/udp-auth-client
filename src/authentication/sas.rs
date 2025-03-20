use std::net::UdpSocket;

use super::package::sas::{SASPackageRequest, SASPackageResponse};

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
        .send(&pack.as_bytes())
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

fn validation(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!")
}

fn status(_socket: &UdpSocket) {
    panic!("not impl!")
}
