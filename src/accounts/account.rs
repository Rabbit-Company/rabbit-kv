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

}