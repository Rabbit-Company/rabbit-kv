use std::time::{SystemTime, UNIX_EPOCH};
use indexmap::IndexMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Result, BufReader, BufWriter};

use super::stats::Stats;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
	Str(String),
	Int(i64)
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
	key: String,
	value: Value,
	expiration: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheItem {
	pub expiration: u64,
	pub value: Value
}

pub struct Cache {
	pub id: Uuid,
	pub cache: IndexMap<String, CacheItem>,
	pub stats: Stats,
	pub persistant: bool,
}

impl Cache {

	pub fn new() -> Self {
		Cache {
			id: Uuid::new_v4(),
			cache: ( IndexMap::new() ),
			stats: ( Stats { writes: 0, reads: 0, deletes: 0, lists: 0 } ),
			persistant: true
		}
	}

	pub fn set(&mut self, key: String, value: Value, ttl: u64){
		let key2: String = key.clone();
		let value2: Value = value.clone();
		let expiration: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() + ttl;
		self.cache.insert(key, CacheItem { expiration, value });
		if self.persistant {
			let mut file: File = OpenOptions::new().write(true).append(true).create(true).open(self.id.to_string()).unwrap();
			let kv: KeyValue = KeyValue { key: key2, value: value2, expiration };
			let line: String = serde_json::to_string(&kv).unwrap() + "\n";
			file.write(line.as_bytes()).ok();
		}
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
		let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
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

	pub fn load(&mut self) -> Result<()>{
		let file: File = File::open(self.id.to_string())?;
		let reader: BufReader<File> = BufReader::new(file);
		for line in reader.lines() {
			let kv: KeyValue = serde_json::from_str(line.as_ref().unwrap())?;
			self.cache.insert(kv.key, CacheItem { expiration: kv.expiration, value: kv.value });
		}
		Ok(())
	}

	pub fn save(&self) -> Result<()>{
		let file: File = OpenOptions::new().write(true).append(false).create(true).open(self.id.to_string())?;
		let mut writer: BufWriter<File> = BufWriter::new(file);
		for (key, cache_item) in &self.cache {
			let kv: KeyValue = KeyValue { key: key.clone(), value: cache_item.value.clone(), expiration: cache_item.expiration };
			let line: String = serde_json::to_string(&kv)?;
			writeln!(writer, "{}\n", line)?;
		}
		Ok(())
	}

}

impl Default for Cache {
	fn default() -> Self {
		Self::new()
	}
}