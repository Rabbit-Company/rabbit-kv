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
	expiration: Instant,
	value: Value
}

pub struct Cache {
	cache: HashMap<String, CacheItem>,
	stats: Stats
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

	pub fn get(&mut self, key: String) -> Option<&CacheItem>{
		self.cache.get(&key)
	}

	pub fn delete(&mut self, key: String){
		self.cache.remove(&key);
	}

	pub fn list(&mut self){
		//self.cache.keys()
	}

	pub fn get_stats(&mut self) -> &Stats{
		&self.stats
	}

}

impl CacheItem {

	pub fn get_value(&self) -> &Value{
		&self.value
	}

	pub fn get_expiration(&self) -> &Instant{
		&self.expiration
	}

}

impl Default for Cache {
	fn default() -> Self {
		Self::new()
	}
}