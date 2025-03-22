use std::str;

use crate::authentication::check::{
    check_gas_request, check_gas_response, check_gas_status, check_gas_validation, TokenType,
};

pub struct GASPackageRequest {
    raw: Vec<u8>,
}

fn add_sas_to_buffer(buf: &mut Vec<u8>, sas: &[&str]) {
    let sas_len = sas.len();
    if sas_len != 3 {
        panic!("not arg!");
    }

    let id = *sas.first().unwrap();
    let nonce = *sas.get(1).unwrap();
    let token = *sas.get(2).unwrap();

    let mut id_bytes = [0u8; 12];
    let mut nonce_bytes = [0u8; 4];
    let mut token_bytes = [0u8; 64];

    let id_as_bytes = id.as_bytes();
    let nonce_as_bytes = nonce.parse::<u32>().unwrap().to_be_bytes();
    let token_as_bytes = token.as_bytes();

    let len = id_as_bytes.len().min(12);
    id_bytes[..len].copy_from_slice(&id_as_bytes[..len]);

    let len = nonce_as_bytes.len().min(4);
    nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..len]);

    let len = token_as_bytes.len().min(64);
    token_bytes[..len].copy_from_slice(&token_as_bytes[..len]);

    buf.extend_from_slice(&id_bytes);
    buf.extend_from_slice(&nonce_bytes);
    buf.extend_from_slice(&token_bytes);
}

impl GASPackageRequest {
    pub fn new(vec_sas: Vec<Vec<&str>>) -> Self {
        let mut buffer = Vec::new();
        let pack_type = TokenType::GroupTokenRequest as u16;
        buffer.extend_from_slice(&pack_type.to_be_bytes());

        let sas_len = vec_sas.len() as u16;
        buffer.extend_from_slice(&sas_len.to_be_bytes());

        vec_sas.iter().for_each(|item| {
            add_sas_to_buffer(&mut buffer, item);
        });

        check_gas_request(&buffer);
        Self { raw: buffer }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.raw
    }
}

pub struct GASPackageResponse {
    raw: Vec<u8>,
    n_sas: usize,
}

impl GASPackageResponse {
    pub fn new(bytes: &[u8], n_sas: usize) -> Self {
        let buffer = bytes.to_vec();

        check_gas_response(&buffer);
        Self { raw: buffer, n_sas }
    }

    pub fn print_gas(&self) {
        for i in 0..self.n_sas {
            let id = str::from_utf8(&self.raw[(4 + 80 * i)..(16 + 80 * i)]).unwrap();
            let nonce = u32::from_be_bytes(self.raw[(16 + 80 * i)..(20 + 80 * i)].try_into().unwrap());
            let token = str::from_utf8(&self.raw[(20 + 80 * i)..(84 + 80 * i)]).unwrap();

            print!("{id}:{nonce}:{token}+");
        }

        let token = str::from_utf8(&self.raw[(4 + 80 * self.n_sas)..]).unwrap();
        println!("{token}");
    }
}

pub struct GASPackageValidation {
    raw: Vec<u8>,
}

impl GASPackageValidation {
    pub fn new(vec_sas: &[&str]) -> Self {
        let mut buffer = Vec::new();
        let pack_type = TokenType::GroupTokenValidation as u16;
        buffer.extend_from_slice(&pack_type.to_be_bytes());

        let sas_len = vec_sas.len() as u16 - 1;
        buffer.extend_from_slice(&sas_len.to_be_bytes());

        for sas in &vec_sas[..sas_len as usize] {
            let item: Vec<&str> = (*sas).split(":").collect();
            add_sas_to_buffer(&mut buffer, &item);
        }

        let token = *vec_sas.last().unwrap();
        buffer.extend_from_slice(token.as_bytes());

        check_gas_validation(&buffer);
        Self { raw: buffer }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.raw
    }
}

pub struct GASPackageStatus {
    raw: Vec<u8>,
    n_sas: usize,
}

impl GASPackageStatus {
    pub fn new(bytes: &[u8], n_sas: usize) -> Self {
        let buffer = bytes.to_vec();

        check_gas_status(&buffer);
        Self { raw: buffer, n_sas }
    }

    pub fn print_status(&self) {
        let status_position = 68 + 80 * self.n_sas;
        let status = u8::from_be_bytes(self.raw[status_position..].try_into().unwrap());
        println!("{status}");
    }
}
