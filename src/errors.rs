use serde::Serialize;
use serde_json::Value;

pub enum Error {
	Success,
	UsernameExists,
	UsernameInvalid,
	EmailExists,
	EmailInvalid,
	PasswordInvalid,
}

#[derive(Serialize)]
pub struct JValue {
	code: u16,
	info: &'static str,
	response: Option<Value>,
}

impl Error {
	pub fn code(&self) -> u16 {
		match self {
			Self::Success => 0,
			Self::UsernameExists => 1,
			Self::UsernameInvalid => 2,
			Self::EmailExists => 3,
			Self::EmailInvalid => 4,
			Self::PasswordInvalid => 5,
		}
	}

	pub fn message(&self) -> &'static str {
		match self {
			Self::Success => "Success",
			Self::UsernameExists => "Username already exists!",
			Self::UsernameInvalid => "Username can only contain lowercase characters, numbers and hyphens. It also needs to start with lowercase character and be between 4 and 30 characters long.",
			Self::EmailExists => "Email already exists!",
			Self::EmailInvalid => "Email is invalid!",
			Self::PasswordInvalid => "Password needs to be hashed with Blake2b. The length of hashed password needs to be 128 characters.",
		}
	}

	pub fn json(&self, response: Option<Value>) -> JValue{
		JValue{ code: Self::code(self), info: Self::message(self), response }
	}

}