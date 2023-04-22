use std::time::{Instant, Duration};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
	pub cache: HashMap<String, CacheItem>
}

impl Cache {

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

}