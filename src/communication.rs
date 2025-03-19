use std::{net::UdpSocket, str};

use crate::package::TokenType;

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

    let mut id = [0u8; 12];
    let bytes = args.first().unwrap().as_bytes();
    let len = bytes.len().min(12);
    id[..len].copy_from_slice(&bytes[..len]);

    let mut pack_itr = vec![0u8; 18];
    let req_type = TokenType::IndividualTokenRequest as u16;
    pack_itr[..2].copy_from_slice(&req_type.to_be_bytes());
    pack_itr[2..14].copy_from_slice(&id);
    pack_itr[14..18].copy_from_slice(&args.get(1).unwrap().parse::<u32>().unwrap().to_le_bytes());

    socket.send(&pack_itr).expect("couldn't send message!");
}

fn response_itr(socket: &UdpSocket) {
    let mut buf = [0; 100];
    let response = match socket.recv(&mut buf) {
        Ok(received) => &buf[..received],
        Err(e) => panic!("recv function failed: {e:?}"),
    };

    let res_type = u16::from_le_bytes(response[..2].try_into().unwrap());
    if TokenType::IndividualTokenResponse as u16 == res_type {
        panic!("invalid response token!");
    }

    let id = str::from_utf8(&response[2..14]).unwrap();
    let nonce = u32::from_le_bytes(response[14..18].try_into().unwrap());
    let token = str::from_utf8(&response[18..]).unwrap();

    println!("{id}:{nonce}:{token}");
}
