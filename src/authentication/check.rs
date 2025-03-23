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

const ERROR_MESSAGES: [&str; 5] = [
    "Error: Invalid message code!",
    "Error: Incorrect message length!",
    "Error: Invalid parameter!",
    "Error: Invalid single token!",
    "Error: ASCII decode error!",
];

fn check_error_code(token_type: u16, error_message: u16) {
    if token_type == TokenType::ErrorMessage as u16 {
        if let Some(&message) = match error_message {
            x if x == ErrorMessage::InvalidMessageCode as u16 => Some(&ERROR_MESSAGES[0]),
            x if x == ErrorMessage::IncorrectMessageLength as u16 => Some(&ERROR_MESSAGES[1]),
            x if x == ErrorMessage::InvalidParameter as u16 => Some(&ERROR_MESSAGES[2]),
            x if x == ErrorMessage::InvalidSingleToken as u16 => Some(&ERROR_MESSAGES[3]),
            x if x == ErrorMessage::AsciiDecodeError as u16 => Some(&ERROR_MESSAGES[4]),
            _ => None,
        } {
            eprintln!("{}", message);
            std::process::exit(0);
        }
    }
}

fn extract_token_info(buf: &[u8]) -> (u16, u16) {
    let token_type = u16::from_be_bytes(buf[..2].try_into().unwrap());
    let error_message = u16::from_be_bytes(buf[2..4].try_into().unwrap());
    (token_type, error_message)
}

pub fn check_sas_request(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::IndividualTokenRequest as u16, "Invalid SAS request token type!");
}

pub fn check_sas_response(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::IndividualTokenResponse as u16, "Invalid SAS response token type!");
}

pub fn check_sas_status(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::IndividualTokenStatus as u16, "Invalid SAS status token type!");
}

pub fn check_sas_validation(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::IndividualTokenValidation as u16, "Invalid SAS validation token type!");
}

pub fn check_gas_request(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::GroupTokenRequest as u16, "Invalid GAS request token type!");
}

pub fn check_gas_response(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::GroupTokenResponse as u16, "Invalid GAS response token type!");
}

pub fn check_gas_status(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::GroupTokenStatus as u16, "Invalid GAS status token type!");
}

pub fn check_gas_validation(buf: &[u8]) {
    let (token_type, error_message) = extract_token_info(buf);
    check_error_code(token_type, error_message);
    assert!(token_type == TokenType::GroupTokenValidation as u16, "Invalid GAS validation token type!");
}
