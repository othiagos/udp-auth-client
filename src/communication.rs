use std::{net::UdpSocket, str};

use crate::package::{Package, TokenType};

pub fn itr(socket: &UdpSocket, args: &[String]) {
    request_itr(socket, args);
    response_itr(socket);
}

pub fn itv(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!")
}

pub fn gtr(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!")
}

pub fn gtv(_socket: &UdpSocket, _args: &[String]) {
    panic!("not impl!")
}

fn request_itr(socket: &UdpSocket, args: &[String]) {
    if args.len() < 2 {
        panic!("few arguments: expected more arguments!");
    }

    let id = args.first().unwrap();
    let nonce = args.get(1).unwrap();
    let pack_request_itr = Package::new_request_itr(id, nonce);

    socket
        .send(&pack_request_itr.to_bytes())
        .expect("couldn't send package!");
}

fn response_itr(socket: &UdpSocket) {
    let mut buf = [0; 100];
    let response = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let pack_responde_itr = Package::new_response_itr(&buf);

    let res_type = u16::from_le_bytes(response[..2].try_into().unwrap());
    if TokenType::IndividualTokenResponse as u16 == res_type {
        panic!("invalid response token!");
    }

    let response = pack_responde_itr.to_bytes();
    let id = str::from_utf8(&response[2..14]).unwrap();
    let nonce = u32::from_be_bytes(response[14..18].try_into().unwrap());
    let token = str::from_utf8(&response[18..]).unwrap();

    println!("{id}:{nonce}:{token}");
}
