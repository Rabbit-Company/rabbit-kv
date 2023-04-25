pub enum Error {
	Success,
  UsernameExists,
}

impl Error {
  pub fn code(&self) -> u8 {
    match self {
			Self::Success => 0,
      Self::UsernameExists => 1,
    }
  }

  pub fn message(&self) -> &'static str {
    match self {
			Self::Success => "Success",
      Self::UsernameExists => "Username already exists!",
    }
  }
}