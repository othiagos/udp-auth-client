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

pub struct ITRes {
    id: [u8; 12],
    nonce: [u8; 4],
}

pub struct ITReq {
    id: [u8; 12],
    nonce: [u8; 4],
    token: [u8; 64],
}

impl Package {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.token_type.to_be_bytes());
        buffer.extend_from_slice(&self.information.get_bytes());
        buffer
    }
}

impl AuthProc for ITRes {
    fn get_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.id);
        buffer.extend_from_slice(&self.nonce);
        buffer
    }
}

impl AuthProc for ITReq {
    fn get_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.id);
        buffer.extend_from_slice(&self.nonce);
        buffer.extend_from_slice(&self.token);
        buffer
    }
}