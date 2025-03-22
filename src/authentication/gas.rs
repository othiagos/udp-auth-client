use std::net::UdpSocket;

use super::package::gas::{GASPackageRequest, GASPackageResponse, GASPackageStatus, GASPackageValidation};

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
    let len = args.first().expect("msg").parse::<usize>().unwrap();

    let vec_sas: Vec<Vec<&str>> = args[1..].iter().map(|sas| make_sas_from_arg(sas)).collect();

    if vec_sas.len() != len {
        panic!("msg");
    }

    let pack = GASPackageRequest::new(vec_sas);

    socket
        .send(pack.as_bytes())
        .expect("couldn't send package!");

    len
}

fn response(socket: &UdpSocket, sas_len: usize) {
    let buf_len = 80 * sas_len + 68;
    let mut buf = vec![0; buf_len];
    buf.resize(buf_len, 0);

    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let pack = GASPackageResponse::new(buf, sas_len);
    pack.print_gas();
}

fn validation(socket: &UdpSocket, args: &[String]) -> usize {
    if args.is_empty() {
        panic!("few arguments: expected more arguments!");
    }

    let sas_values: Vec<&str> = args.first().unwrap().split("+").collect();
    let pack = GASPackageValidation::new(&sas_values);

    socket
        .send(pack.as_bytes())
        .expect("couldn't send package!");

    sas_values.len() - 1
}

fn status(socket: &UdpSocket, sas_len: usize) {
    let buf_len = 80 * sas_len + 69;
    let mut buf = vec![0; buf_len];
    let buf = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let pack = GASPackageStatus::new(buf, sas_len);
    pack.print_status();
}
