use regex::Regex;
use once_cell::sync::Lazy;

pub static USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-z][a-z0-9\\-]{3,29})$").unwrap());
pub static EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9.!#$%&'*+\\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap());
pub static PASSWORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([a-z0-9]{128})$").unwrap());