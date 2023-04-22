use std::collections::HashMap;

use crate::caches::cache;

pub struct Account {
	pub username: String,
	pub token: String,
	pub caches: HashMap<String, cache::Cache>
}

impl Account {

	pub fn new(username: String, token: String) -> Self {
		Account { username, token, caches: HashMap::new() }
	}

	pub fn authorize(&self, username: &str, token: &str) -> bool{
		self.username == username && self.token == token
	}

	pub fn cache(&self, key: &str) -> Option<&cache::Cache>{
		self.caches.get(key)
	}

}