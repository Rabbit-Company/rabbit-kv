use std::collections::HashMap;

use crate::caches::cache::Cache;

pub struct Account {
	pub username: String,
	pub token: String,
	pub caches: HashMap<String, Cache>
}