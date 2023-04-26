use std::{collections::HashMap, time::SystemTime, time::UNIX_EPOCH};

use crate::caches::cache;

#[derive(Clone)]
pub struct Account {
	pub username: String,
	pub password: String,
	pub email: String,
	pub caches: HashMap<String, cache::Cache>,
	pub created: u64,
	pub accessed: u64
}

impl Account {

	pub fn new(username: String, password: String, email: String) -> Self {
		let created: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
		Account { username, password, email, caches: HashMap::new(), created, accessed: created }
	}

	pub fn authorize(&self, username: &str, password: &str) -> bool{
		self.username == username && self.password == password
	}

	pub fn cache(&self, key: &str) -> Option<&cache::Cache>{
		self.caches.get(key)
	}

}