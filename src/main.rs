use cache::Cache;
use std::collections::HashMap;
pub mod cache;

fn main() {
	let mut cache: Cache = Cache{ cache: HashMap::new() };

	cache.set("test123".to_string(), cache::Value::Str("hello".to_string()), 60);
	cache.set("test1234".to_string(), cache::Value::Int(64), 60);

	let value: &cache::Value = &cache.get("test1234".to_string()).unwrap().value;

	let serialized: String = serde_json::to_string(value).unwrap();
	println!("{}", serialized);
}