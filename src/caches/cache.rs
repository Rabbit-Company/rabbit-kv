use std::time::{Instant, Duration};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::stats::Stats;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
	Str(String),
	Int(i64)
}

pub struct CacheItem {
	pub expiration: Instant,
	pub value: Value
}

pub struct Cache {
	pub cache: HashMap<String, CacheItem>,
	pub stats: Stats
}

impl Cache {

	pub fn new() -> Self {
		Cache {
			cache: ( HashMap::new() ),
			stats: ( Stats { writes: 0, reads: 0, deletes: 0, lists: 0 } )
		}
	}

	pub fn set(&mut self, key: String, value: Value, ttl: u64){
		self.cache.insert(key, CacheItem { expiration: Instant::now() + Duration::from_secs(ttl), value });
	}

	pub fn get(&self, key: &str) -> Option<&CacheItem>{
		self.cache.get(key)
	}

	pub fn delete(&mut self, key: &str){
		self.cache.remove(key);
	}

	pub fn list(&self){
		//self.cache.keys()
	}

}

impl Default for Cache {
	fn default() -> Self {
		Self::new()
	}
}