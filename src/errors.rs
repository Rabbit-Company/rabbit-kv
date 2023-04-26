pub enum Error {
	Success,
  UsernameExists,
	UsernameInvalid,
	EmailInvalid,
	PasswordInvalid,
}

impl Error {
  pub fn code(&self) -> u8 {
    match self {
			Self::Success => 0,
      Self::UsernameExists => 1,
			Self::UsernameInvalid => 2,
			Self::EmailInvalid => 3,
			Self::PasswordInvalid => 4,
    }
  }

  pub fn message(&self) -> &'static str {
    match self {
			Self::Success => "Success",
      Self::UsernameExists => "Username already exists!",
			Self::UsernameInvalid => "Username can only contain lowercase characters, numbers and hyphens. It also needs to start with lowercase character and be between 4 and 30 characters long.",
			Self::EmailInvalid => "Email is invalid!",
			Self::PasswordInvalid => "Password needs to be hashed with Blake2b. The length of hashed password needs to be 128 characters.",
		}
  }
}