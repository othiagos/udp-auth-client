use std::str;

use crate::authentication::check::{check_sas_request, check_sas_response, check_sas_status, check_sas_validation, TokenType};

const SIZE_ID_LEN: usize = 12;
const SIZE_NONCE_LEN: usize = 4;
const SIZE_TOKEN_LEN: usize = 64;
const ID_OFFSET: usize = 2;
const NONCE_OFFSET: usize = 14;
const TOKEN_OFFSET: usize = 18;
const STATUS_OFFSET: usize = 82;

pub struct SASPackageRequest {
    raw: Vec<u8>,
}

impl SASPackageRequest {
    pub fn new(id: &str, nonce: &str) -> Self {
        let mut buffer = Vec::new();
        let pack_type = TokenType::IndividualTokenRequest as u16;

        let mut id_bytes = [0u8; SIZE_ID_LEN];
        let mut nonce_bytes = [0u8; SIZE_NONCE_LEN];

        let id_as_bytes = id.as_bytes();

        let nonce_as_bytes = match nonce.parse::<u32>() {
            Ok(number) => number.to_be_bytes(),
            Err(e) => {
                eprintln!("Invalid nonce number: {:?}", e.to_string());
                std::process::exit(1);
            }
        };
    
        let len = id_as_bytes.len().min(SIZE_ID_LEN);
        id_bytes[..len].copy_from_slice(&id_as_bytes[..len]);

        let len = nonce_as_bytes.len().min(SIZE_NONCE_LEN);
        nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..len]);

        buffer.extend_from_slice(&pack_type.to_be_bytes());
        buffer.extend_from_slice(&id_bytes);
        buffer.extend_from_slice(&nonce_bytes);

        check_sas_request(&buffer);
        Self { raw: buffer }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.raw
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
        let id = str::from_utf8(&self.raw[ID_OFFSET..ID_OFFSET + SIZE_ID_LEN]).unwrap();
        let nonce = u32::from_be_bytes(self.raw[NONCE_OFFSET..NONCE_OFFSET + SIZE_NONCE_LEN].try_into().unwrap());
        let token = str::from_utf8(&self.raw[TOKEN_OFFSET..]).unwrap();

        println!("{id}:{nonce}:{token}");
    }
}

pub struct SASPackageStatus {
    raw: Vec<u8>,
}

impl SASPackageStatus {
    pub fn new(bytes: &[u8]) -> Self {
        let buffer = bytes.to_vec();

        check_sas_status(&buffer);
        Self { raw: buffer }
    }

    pub fn print_status(&self) {
        let status = u8::from_be_bytes(self.raw[STATUS_OFFSET..].try_into().unwrap());
        println!("{status}");
    }
}

pub struct SASPackageValidation {
    raw: Vec<u8>,
}

impl SASPackageValidation {
    pub fn new(id: &str, nonce: &str, token: &str) -> Self {
        let mut buffer = Vec::new();
    
        let pack_type = TokenType::IndividualTokenValidation as u16;
        let mut id_bytes = [0u8; SIZE_ID_LEN];
        let mut nonce_bytes = [0u8; SIZE_NONCE_LEN];
        let mut token_bytes = [0u8; SIZE_TOKEN_LEN];

        let id_as_bytes = id.as_bytes();
        let token_as_bytes = token.as_bytes();

        let nonce_as_bytes = match nonce.parse::<u32>() {
            Ok(number) => number.to_be_bytes(),
            Err(e) => {
                eprintln!("Invalid nonce number: {:?}", e.to_string());
                std::process::exit(1);
            }
        };

        let len = id_as_bytes.len().min(SIZE_ID_LEN);
        id_bytes[..len].copy_from_slice(&id_as_bytes[..len]);

        let len = nonce_as_bytes.len().min(SIZE_NONCE_LEN);
        nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..len]);

        let len = token_as_bytes.len().min(SIZE_TOKEN_LEN);
        token_bytes[..len].copy_from_slice(&token_as_bytes[..len]);

        buffer.extend_from_slice(&pack_type.to_be_bytes());
        buffer.extend_from_slice(&id_bytes);
        buffer.extend_from_slice(&nonce_bytes);
        buffer.extend_from_slice(&token_bytes);

        check_sas_validation(&buffer);
        Self { raw: buffer }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.raw
    }
}
