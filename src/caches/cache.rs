use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::fs;
use tokio::io::Result;

use super::stats::Stats;
use crate::utils::current_time;

#[derive(Debug, Serialize, Deserialize)]
struct CacheItemSmall {
	pub v: serde_json::Value,
	pub e: u128
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheItem {
	pub expiration: u128,
	pub value: serde_json::Value
}

#[derive(Clone, Default)]
pub struct Cache {
	pub cache: IndexMap<String, CacheItem>,
	pub stats: Stats,
	pub persistant: bool,
	pub path: String
}

impl Cache {

	pub fn new(persistant: bool, path: String) -> Self {
		Cache {
			cache: IndexMap::new(),
			stats: Stats::default(),
			persistant,
			path
		}
	}

	pub fn set(&mut self, key: String, value: serde_json::Value, ttl: u128){
		self.stats.writes += 1;
		let expiration: u128 = current_time() + ttl;
		self.cache.insert(key, CacheItem { expiration, value });
	}

	pub fn get(&mut self, key: &str) -> Option<&CacheItem>{
		self.stats.reads += 1;
    self.cache.get(key).filter(|&item| item.expiration > current_time())
	}

	pub fn delete(&mut self, key: &str){
		self.stats.deletes += 1;
		self.cache.swap_remove(key);
	}

	pub fn list(&mut self, limit: usize, cursor: usize, prefix: &str) -> Vec<&String>{
		self.stats.lists += 1;
		self.cache.keys().filter(move |k: &&String| k.starts_with(prefix)).enumerate().skip(cursor).take(limit).map(|(_i, k)| k).collect()
	}

	pub fn delete_expired_items(&mut self){
		let mut expired_keys: Vec<String> = Vec::new();
		for (key, cache_item) in self.cache.iter() {
			if current_time() >= cache_item.expiration {
				expired_keys.push(key.clone());
			}
		}
		for key in expired_keys {
			self.cache.swap_remove(&key);
		}
	}

	pub async fn load(&mut self) -> Result<()>{
		match read_cache_from_file(&self.path).await {
			Ok(cache_map) => {
				let cur_time: u128 = current_time();
				for (key, value) in cache_map {
					if value.e < cur_time { continue; }
					let cache_item = CacheItem {
							expiration: value.e,
							value: value.v,
					};
					self.cache.insert(key, cache_item);
				}
				Ok(())
			},
			Err(err) => Err(err),
		}
	}

	pub async fn save(&self) -> Result<()>{
		let mut cache_map: HashMap<String, CacheItemSmall> = HashMap::new();
		let cur_time: u128 = current_time();
		for (key, value) in &self.cache {
			if value.expiration < cur_time { continue; }
			cache_map.insert(key.clone(), CacheItemSmall { v: value.value.clone(), e: value.expiration });
		}
		let json_str = serde_json::to_string_pretty(&cache_map).unwrap();
		write_cache_to_file(&self.path, &json_str).await;
		Ok(())
	}

}

async fn read_cache_from_file(path: &str) -> Result<HashMap<String, CacheItemSmall>> {
	let json_str: String = fs::read_to_string(format!("{}/cache.json", path)).await?;
  Ok(serde_json::from_str(&json_str)?)
}

async fn write_cache_to_file(path: &str, json_str: &str) {
	fs::write(format!("{}/cache.json", path), json_str).await.ok();
}