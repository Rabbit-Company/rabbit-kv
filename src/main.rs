pub mod caches;
pub mod accounts;

use crate::caches::cache;

fn main() {
	let mut cache: cache::Cache = cache::Cache::new();

	cache.set("test1".to_string(), cache::Value::String("hello".to_string()), 60);
	cache.set("test2".to_string(), cache::Value::Number(64), 60);
	cache.set("test3".to_string(), cache::Value::BigNumber(9007199254740991), 60);
	cache.set("test4".to_string(), cache::Value::Decimal(3.43453), 60);
	cache.set("test5".to_string(), cache::Value::Boolean(true), 60);

	cache.delete("test2");

	let value: &cache::Value = &cache.get("test1").unwrap().value;

	let serialized: String = serde_json::to_string(value).unwrap();
	println!("{}", serialized);

	println!("{:?}", cache.list(1000, 0, "test"));
}