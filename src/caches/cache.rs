use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::io;

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
	pub path: String,
	pub preserve_order: bool
}

impl Cache {

	pub fn new(path: String, preserve_order: bool) -> Self {
		Cache {
			cache: IndexMap::new(),
			stats: Stats::default(),
			path,
			preserve_order
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
		if self.preserve_order {
			self.cache.shift_remove(key);
		}else{
			self.cache.swap_remove(key);
		}
	}

	pub fn list(&mut self, limit: usize, cursor: usize, prefix: &str) -> Vec<&String>{
		self.stats.lists += 1;
		self.cache.keys().filter(move |k: &&String| k.starts_with(prefix)).enumerate().skip(cursor).take(limit).map(|(_i, k)| k).collect()
	}

	pub fn clean(&mut self){
		let current_time: u128 = current_time();
		self.cache.retain(|_, cache_item| current_time < cache_item.expiration)
	}

	pub fn load(&mut self) -> io::Result<()>{
		match read_cache_from_file(&self.path) {
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

	pub fn save(&self) -> io::Result<()>{
		let mut cache_map: HashMap<String, CacheItemSmall> = HashMap::new();
		let cur_time: u128 = current_time();
		for (key, value) in &self.cache {
			if value.expiration < cur_time { continue; }
			cache_map.insert(key.clone(), CacheItemSmall { v: value.value.clone(), e: value.expiration });
		}
		let json_str = serde_json::to_string_pretty(&cache_map).unwrap();
		write_cache_to_file(&self.path, &json_str)
	}

}

fn read_cache_from_file(path: &str) -> io::Result<HashMap<String, CacheItemSmall>> {
	let json_str: String = fs::read_to_string(format!("{}/cache.json", path))?;
  Ok(serde_json::from_str(&json_str)?)
}

fn write_cache_to_file(path: &str, json_str: &str) -> io::Result<()> {
	fs::write(format!("{}/cache.json", path), json_str)?;
	Ok(())
}