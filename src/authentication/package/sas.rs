use std::str;

use crate::authentication::check::{check_sas_request, check_sas_response, TokenType};

pub struct SASPackageRequest {
    raw: Vec<u8>,
}

impl SASPackageRequest {
    pub fn new(id: &str, nonce: &str) -> Self {
        let mut buffer = Vec::new();
        let pack_type = TokenType::IndividualTokenRequest as u16;

        let mut id_bytes = [0u8; 12];
        let mut nonce_bytes = [0u8; 4];

        let id_as_bytes = id.as_bytes();
        let nonce_as_bytes = nonce.parse::<u32>().unwrap().to_be_bytes();

        let len = id_as_bytes.len().min(12);
        id_bytes[..len].copy_from_slice(&id_as_bytes[..id_as_bytes.len().min(12)]);

        let len = nonce_as_bytes.len().min(4);
        nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..nonce_as_bytes.len().min(4)]);

        buffer.extend_from_slice(&pack_type.to_be_bytes());
        buffer.extend_from_slice(&id_bytes);
        buffer.extend_from_slice(&nonce_bytes);

        check_sas_request(&buffer);
        Self { raw: buffer }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}

pub struct SASPackageResponse {
    raw: Vec<u8>,
}

impl SASPackageResponse {
    pub fn new(bytes: &[u8]) -> Self {
        let buffer = bytes.to_vec();

        check_sas_response(&buffer);
        Self { raw: buffer }
    }

    pub fn print_sas(&self) {
        let id = str::from_utf8(&self.raw[2..14]).unwrap();
        let nonce = u32::from_be_bytes(self.raw[14..18].try_into().unwrap());
        let token = str::from_utf8(&self.raw[18..]).unwrap();

        println!("{id}:{nonce}:{token}");
    }
}

pub struct SASPackageStatus {
    raw: Vec<u8>,
}

impl SASPackageStatus {
    pub fn new(bytes: &[u8]) -> Self {
        panic!("not impl!")
    }
}

pub struct SASPackageValidation {
    raw: Vec<u8>,
}

impl SASPackageValidation {
    pub fn new(bytes: &[u8]) -> Self {
        panic!("not impl!")
    }
}
