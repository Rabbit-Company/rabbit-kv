use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

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
		let key2: String = key.clone();
		let value2: serde_json::Value = value.clone();
		self.stats.writes += 1;
		let expiration: u128 = current_time() + ttl;
		self.cache.insert(key, CacheItem { expiration, value });
		if self.persistant {
			let mut cache_map: HashMap<String, CacheItemSmall> = read_cache_from_file(&self.path);
			cache_map.insert(key2, CacheItemSmall { v: value2, e: expiration });
			let json_str = serde_json::to_string_pretty(&cache_map).unwrap();
			write_cache_to_file(&self.path, &json_str);
		}
	}

	pub fn get(&mut self, key: &str) -> Option<&CacheItem>{
		self.stats.reads += 1;
    self.cache.get(key).filter(|&item| item.expiration > current_time())
	}

	pub fn delete(&mut self, key: &str){
		self.stats.deletes += 1;
		self.cache.swap_remove(key);
		if self.persistant {
			self.save().ok();
		}
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

	pub fn load(&mut self) -> Result<()>{
		let cache_map: HashMap<String, CacheItemSmall> = read_cache_from_file(&self.path);
		for (key, value) in cache_map {
			let cache_item = CacheItem {
					expiration: value.e,
					value: value.v,
			};
			self.cache.insert(key, cache_item);
		}
		/*
		let file: File = File::open(format!("{}/cache.jsonl", self.path))?;
		let reader: BufReader<File> = BufReader::new(file);
		for line in reader.lines() {
			let kv: KeyValue = serde_json::from_str(line.as_ref().unwrap())?;
			self.cache.insert(kv.k, CacheItem { expiration: kv.e, value: kv.v });
		}
		*/
		Ok(())
	}

	pub fn save(&self) -> Result<()>{
		let mut cache_map: HashMap<String, CacheItemSmall> = HashMap::new();
		for (key, value) in &self.cache {
			cache_map.insert(key.clone(), CacheItemSmall { v: value.value.clone(), e: value.expiration });
		}
		let json_str = serde_json::to_string_pretty(&cache_map).unwrap();
		write_cache_to_file(&self.path, &json_str);

		/*
		let file: File = OpenOptions::new().write(true).truncate(true).create(true).open(format!("{}/cache.jsonl", self.path))?;
		let mut writer: BufWriter<File> = BufWriter::new(file);
		for (key, cache_item) in &self.cache {
			let kv: KeyValue = KeyValue { k: key.clone(), v: cache_item.value.clone(), e: cache_item.expiration };
			let line: String = serde_json::to_string(&kv)?;
			writeln!(writer, "{}", line)?;
		}
		*/
		Ok(())
	}

}

fn read_cache_from_file(path: &str) -> HashMap<String, CacheItemSmall> {
	let mut file = File::open(format!("{}/cache.json", path)).expect("Failed to open cache file!");
	let mut json_str = String::new();
	file.read_to_string(&mut json_str).expect("Failed to read cache from a file!");

	serde_json::from_str(&json_str).expect("Failed to read cache from a file!")
}

fn write_cache_to_file(path: &str, json_str: &str) {
	let mut file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open(format!("{}/cache.json", path))
		.expect("Failed to open cache file!");

	file.write_all(json_str.as_bytes()).expect("Failed to write cache to file!");
}