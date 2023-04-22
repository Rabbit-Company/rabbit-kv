pub mod caches;
pub mod accounts;

use crate::caches::cache;

fn main() {
	let mut cache: cache::Cache = cache::Cache::new();

	cache.set("test123".to_string(), cache::Value::Str("hello".to_string()), 60);
	cache.set("test1234".to_string(), cache::Value::Int(64), 60);

	let value: &cache::Value = cache.get("test1234".to_string()).unwrap().get_value();

	let serialized: String = serde_json::to_string(value).unwrap();
	println!("{}", serialized);
}