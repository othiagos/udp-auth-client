use std::str;

use crate::authentication::check::{
    check_gas_request, check_gas_response, check_gas_status, check_gas_validation, TokenType,
};

const SIZE_ID_LEN: usize = 12;
const SIZE_NONCE_LEN: usize = 4;
const SIZE_TOKEN_LEN: usize = 64;
const PACK_HEAD_SIZE: usize = 4;
const STATUS_OFFSET: usize = 68;
const SAS_DATA_SIZE: usize = 80;

fn add_sas_to_buffer(buf: &mut Vec<u8>, sas: &[&str]) {
    const REQUIRED_SAS_LEN: usize = 3;

    let sas_len = sas.len();
    if sas_len != REQUIRED_SAS_LEN {
        panic!("Incorrect number of arguments. Expected {REQUIRED_SAS_LEN}, got {sas_len}.");
    }

    let id = sas[0];
    let nonce = sas[1];
    let token = sas[2];

    let mut id_bytes = [0u8; SIZE_ID_LEN];
    let mut nonce_bytes = [0u8; SIZE_NONCE_LEN];
    let mut token_bytes = [0u8; SIZE_TOKEN_LEN];

    let id_as_bytes = id.as_bytes();
    let nonce_as_bytes = nonce.parse::<u32>().unwrap().to_be_bytes();
    let token_as_bytes = token.as_bytes();

    let len = id_as_bytes.len().min(SIZE_ID_LEN);
    id_bytes[..len].copy_from_slice(&id_as_bytes[..len]);

    let len = nonce_as_bytes.len().min(SIZE_NONCE_LEN);
    nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..len]);

    let len = token_as_bytes.len().min(SIZE_TOKEN_LEN);
    token_bytes[..len].copy_from_slice(&token_as_bytes[..len]);

    buf.extend_from_slice(&id_bytes);
    buf.extend_from_slice(&nonce_bytes);
    buf.extend_from_slice(&token_bytes);
}

pub struct GASPackageRequest {
    raw: Vec<u8>,
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
            // Calculating the range for the ID
            let id_start = PACK_HEAD_SIZE + SAS_DATA_SIZE * i;
            let id_end = id_start + SIZE_ID_LEN;
            let id_slice = &self.raw[id_start..id_end];
            let id = str::from_utf8(id_slice).unwrap();
            
            // Calculating the range for the nonce
            let nonce_start = id_end;
            let nonce_end = nonce_start + SIZE_NONCE_LEN;
            let nonce_slice = &self.raw[nonce_start..nonce_end];
            let nonce = u32::from_be_bytes(nonce_slice.try_into().unwrap());
            
            // Calculating the range for the token
            let token_start = nonce_end;
            let token_end = token_start + SIZE_TOKEN_LEN;
            let token_slice = &self.raw[token_start..token_end];
            let token = str::from_utf8(token_slice).unwrap();
            
            print!("{id}:{nonce}:{token}+");
        }

        // Processing the last token
        let last_token_start = PACK_HEAD_SIZE + SAS_DATA_SIZE * self.n_sas;
        let last_token_slice = &self.raw[last_token_start..];
        let last_token = str::from_utf8(last_token_slice).unwrap();
        println!("{last_token}");
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
            let item: Vec<&str> = sas.split(":").collect();
            add_sas_to_buffer(&mut buffer, &item);
        }

        let token = vec_sas.last().unwrap();
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
        let status_position = STATUS_OFFSET + SAS_DATA_SIZE * self.n_sas;
        let status = u8::from_be_bytes(self.raw[status_position..].try_into().unwrap());
        println!("{status}");
    }
}
