use std::net::UdpSocket;

pub fn gtr(socket: &UdpSocket, args: &[String]) {
    request(socket, args);
    response(socket);
}

pub fn gtv(socket: &UdpSocket, args: &[String]) {
    validation(socket, args);
    status(socket);
}

fn request(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!");
}

fn response(_socket: &UdpSocket) {
    panic!("not impl!");
}

fn validation(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!");
}

fn status(_socket: &UdpSocket) {
    panic!("not impl!");
}
