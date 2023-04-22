use std::collections::HashMap;

use crate::caches::cache;

pub struct Account {
	username: String,
	token: String,
	caches: HashMap<String, cache::Cache>
}

impl Account {

	pub fn new(username: String, token: String) -> Self {
		Account { username, token, caches: HashMap::new() }
	}

	pub fn get_username(&mut self) -> &String{
		&self.username
	}

	pub fn get_token(&mut self) -> &String{
		&self.token
	}

}