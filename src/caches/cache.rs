use std::time::{Instant, Duration};
use indexmap::IndexMap;
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
	pub cache: IndexMap<String, CacheItem>,
	pub stats: Stats
}

impl Cache {

	pub fn new() -> Self {
		Cache {
			cache: ( IndexMap::new() ),
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

	pub fn list<'a>(&'a self, limit: usize, cursor: usize, prefix: &'a str) -> Vec<&'a String>{
		self.cache.keys().filter(move |k: &&String| k.starts_with(prefix)).enumerate().skip(cursor).take(limit).map(|(_i, k)| k).collect()
	}

	pub fn delete_expired_items(&mut self){
		let now: Instant = Instant::now();
		let mut expired_keys: Vec<String> = Vec::new();
		for (key, cache_item) in self.cache.iter() {
			if now >= cache_item.expiration {
				expired_keys.push(key.clone());
			}
		}
		for key in expired_keys {
			self.cache.remove(&key);
		}
	}

}

impl Default for Cache {
	fn default() -> Self {
		Self::new()
	}
}