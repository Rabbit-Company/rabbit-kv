use std::time::{SystemTime, UNIX_EPOCH};
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Result, BufReader, BufWriter};

use super::stats::Stats;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
	k: String,
	v: serde_json::Value,
	e: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheItem {
	pub expiration: u64,
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

	pub fn set(&mut self, key: String, value: serde_json::Value, ttl: u64){
		let key2: String = key.clone();
		let value2: serde_json::Value = value.clone();
		self.stats.writes += 1;
		let expiration: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() + ttl;
		self.cache.insert(key, CacheItem { expiration, value });
		if self.persistant {
			let mut file: File = OpenOptions::new().append(true).create(true).open(format!("{}/cache.jsonl", self.path)).unwrap();
			let kv: KeyValue = KeyValue { k: key2, v: value2, e: expiration };
			let line: String = serde_json::to_string(&kv).unwrap() + "\n";
			file.write_all(line.as_bytes()).ok();
			file.flush().ok();
		}
	}

	pub fn get(&mut self, key: &str) -> Option<&CacheItem>{
		self.stats.reads += 1;
		self.cache.get(key)
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
		let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
		let mut expired_keys: Vec<String> = Vec::new();
		for (key, cache_item) in self.cache.iter() {
			if now >= cache_item.expiration {
				expired_keys.push(key.clone());
			}
		}
		for key in expired_keys {
			self.cache.swap_remove(&key);
		}
	}

	pub fn load(&mut self) -> Result<()>{
		let file: File = File::open(format!("{}/cache.jsonl", self.path))?;
		let reader: BufReader<File> = BufReader::new(file);
		for line in reader.lines() {
			let kv: KeyValue = serde_json::from_str(line.as_ref().unwrap())?;
			self.cache.insert(kv.k, CacheItem { expiration: kv.e, value: kv.v });
		}
		Ok(())
	}

	pub fn save(&self) -> Result<()>{
		let file: File = OpenOptions::new().write(true).truncate(true).create(true).open(format!("{}/cache.jsonl", self.path))?;
		let mut writer: BufWriter<File> = BufWriter::new(file);
		for (key, cache_item) in &self.cache {
			let kv: KeyValue = KeyValue { k: key.clone(), v: cache_item.value.clone(), e: cache_item.expiration };
			let line: String = serde_json::to_string(&kv)?;
			writeln!(writer, "{}", line)?;
		}
		Ok(())
	}

}