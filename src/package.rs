use std::str;

pub enum TokenType {
    IndividualTokenRequest = 1,
    IndividualTokenResponse = 2,
    _IndividualTokenValidation = 3,
    _IndividualTokenStatus = 4,
    _GroupTokenRequest = 5,
    _GroupTokenResponse = 6,
    _GroupTokenValidation = 7,
    _GroupTokenStatus = 8,
    _ErrorMessage = 256,
}

trait AuthProc {
    fn get_bytes(&self) -> Vec<u8>;
}

pub struct Package {
    token_type: u16,
    information: Box<dyn AuthProc>,
}
pub struct ITReq {
    id: [u8; 12],
    nonce: [u8; 4],
}

pub struct ITRes {
    id: String,
    nonce: u32,
    token: String,
}

impl Package {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.token_type.to_be_bytes());
        buffer.extend_from_slice(&self.information.get_bytes());
        buffer
    }

    pub fn new_request_itr(id: &String, nonce: &String) -> Self {
        let mut id_bytes = [0u8; 12]; // Initializes a zeroed array of 12 bytes
        let mut nonce_bytes = [0u8; 4]; // Initializes a zeroed array of 4 bytes

        let id_as_bytes = id.as_bytes(); // Converts the string to bytes
        let nonce_as_bytes = nonce.as_bytes(); // Converts the string to bytes

        // Copies the string bytes into the array, ensuring it does not exceed 12 bytes
        let len = id_as_bytes.len().min(12);
        id_bytes[..len].copy_from_slice(&id_as_bytes[..len]);

        // Copies the string bytes into the array, ensuring it does not exceed 4 bytes
        let len = nonce_as_bytes.len().min(4);
        nonce_bytes[..len].copy_from_slice(&nonce_as_bytes[..len]);

        Self {
            token_type: TokenType::IndividualTokenRequest as u16,
            information: Box::new(ITReq::new(id_bytes, nonce_bytes)),
        }
    }

    pub fn new_response_itr(bytes: &[u8]) -> Self {
        let response_type = u16::from_be_bytes(bytes[..2].try_into().unwrap());
        if TokenType::IndividualTokenResponse as u16 != response_type {
            panic!("invalid response token!");
        }

        let id = str::from_utf8(&bytes[2..14]).unwrap();
        let nonce = u32::from_be_bytes(bytes[14..18].try_into().unwrap());
        let token = str::from_utf8(&bytes[18..]).unwrap();

        Self {
            token_type: TokenType::IndividualTokenRequest as u16,
            information: Box::new(ITRes::new(id.to_string(), nonce, token.to_string())),
        }
    }
}

impl ITReq {
    fn new(id: [u8; 12], nonce: [u8; 4]) -> Self {
        Self { id, nonce }
    }
}

impl ITRes {
    fn new(id: String, nonce: u32, token: String) -> Self {
        Self { id, nonce, token }
    }
}

impl AuthProc for ITReq {
    fn get_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.id);
        buffer.extend_from_slice(&self.nonce);
        buffer
    }
}

impl AuthProc for ITRes {
    fn get_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(self.id.as_bytes());
        buffer.extend_from_slice(&self.nonce.to_be_bytes());
        buffer.extend_from_slice(self.token.as_bytes());
        buffer
    }
}
