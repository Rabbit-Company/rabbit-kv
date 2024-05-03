use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Error {
	pub code: u64,
	pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorCode {
	Success = 0,
	InvalidToken = 1000,
	InvalidNumber = 1001,
	InvalidInteger = 1002,
	IntegerOverflow = 1003,
	InvalidData = 1004,
	InvalidPayload = 1005,
}

impl ErrorCode {
	pub fn message(&self) -> String {
		match self {
			ErrorCode::Success => "success".to_string(),
			ErrorCode::InvalidToken => "Provided token is incorrect!".to_string(),
			ErrorCode::InvalidNumber => "Value is not a number!".to_string(),
			ErrorCode::InvalidInteger => "Value is not an integer!".to_string(),
			ErrorCode::IntegerOverflow => "Integer overflow occurred!".to_string(),
			ErrorCode::InvalidData => "Invalid data!".to_string(),
			ErrorCode::InvalidPayload => "Invalid payload!".to_string(),
		}
	}
}

impl Error {
	pub fn new(code: u64, message: &str) -> Self {
		Error {
			code,
			message: message.to_string(),
		}
	}

	pub fn from_code(error_code: ErrorCode) -> Self {
		Error::new(error_code.clone() as u64, &error_code.message())
	}
}