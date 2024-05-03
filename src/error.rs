use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Error {
	pub code: u64,
	pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ErrorCode {
	Success,
	InvalidToken,
	InvalidNumber,
	InvalidInteger,
	IntegerOverflow,
	InvalidData,
	InvalidPayload,
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
		let code = match error_code {
			ErrorCode::Success => 0,
			ErrorCode::InvalidToken => 1000,
			ErrorCode::InvalidNumber => 1001,
			ErrorCode::InvalidInteger => 1002,
			ErrorCode::IntegerOverflow => 1003,
			ErrorCode::InvalidData => 1004,
			ErrorCode::InvalidPayload => 1005,
		};
		Error::new(code, &error_code.message())
	}
}