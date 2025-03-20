pub enum TokenType {
    IndividualTokenRequest = 1,
    IndividualTokenResponse = 2,
    IndividualTokenValidation = 3,
    IndividualTokenStatus = 4,
    GroupTokenRequest = 5,
    GroupTokenResponse = 6,
    GroupTokenValidation = 7,
    GroupTokenStatus = 8,
    ErrorMessage = 256,
}

pub enum ErrorMessage {
    InvalidMessageCode = 1,
    IncorrectMessageLength = 2,
    InvalidParameter = 3,
    InvalidSingleToken = 4,
    AsciiDecodeError = 5,
}

const INVALID_MESSAGE_CODE: &str = "Invalid message code!";
const INCORRECT_MESSAGE_LENGTH: &str = "Incorrect message length!";
const INVALID_PARAMETER: &str = "Invalid parameter!";
const INVALID_SINGLE_TOKEN: &str = "Invalid single token!";
const ASCII_DECODE_ERROR: &str = "ASCII decode error!";

fn check_error_code(token_type: u16, error_message: u16) {
    if TokenType::ErrorMessage as u16 == token_type {
        match error_message {
            x if x == ErrorMessage::InvalidMessageCode as u16 => panic!("{}", INVALID_MESSAGE_CODE),
            x if x == ErrorMessage::IncorrectMessageLength as u16 => panic!("{}", INCORRECT_MESSAGE_LENGTH),
            x if x == ErrorMessage::InvalidParameter as u16 => panic!("{}", INVALID_PARAMETER),
            x if x == ErrorMessage::InvalidSingleToken as u16 => panic!("{}", INVALID_SINGLE_TOKEN),
            x if x == ErrorMessage::AsciiDecodeError as u16 => panic!("{}", ASCII_DECODE_ERROR),
            _ => (), // Default case to avoid unexpected panics
        }
    }
}

pub fn check_sas_request(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::IndividualTokenRequest as u16 == token_type,
        "Invalid SAS request token type!"
    );
}

pub fn check_sas_response(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::IndividualTokenResponse as u16 == token_type,
        "Invalid SAS response token type!"
    );
}

pub fn check_sas_status(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::IndividualTokenStatus as u16 == token_type,
        "Invalid SAS status token type!"
    );
}

pub fn check_sas_validation(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::IndividualTokenValidation as u16 == token_type,
        "Invalid SAS validation token type!"
    );
}

pub fn check_gas_request(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::GroupTokenRequest as u16 == token_type,
        "Invalid GAS request token type!"
    );
}

pub fn check_gas_response(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::GroupTokenResponse as u16 == token_type,
        "Invalid GAS response token type!"
    );
}

pub fn check_gas_status(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::GroupTokenStatus as u16 == token_type,
        "Invalid GAS status token type!"
    );
}

pub fn check_gas_validation(buf: &[u8]) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());

    check_error_code(token_type, error_message);

    assert!(
        TokenType::GroupTokenValidation as u16 == token_type,
        "Invalid GAS validation token type!"
    );
}
