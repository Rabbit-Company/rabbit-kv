pub mod caches;
pub mod accounts;

use std::fs;
use serde_json::json;

use crate::{caches::cache, accounts::Accounts};

fn main() {

	let mut accounts: Accounts = Accounts::new();
	accounts.import().ok();

	fs::create_dir_all("/var/lib/rabbitkv/storage").expect("Permission denied. Please run program with root user.");

	let mut cache: cache::Cache = cache::Cache::new("".to_string());

	cache.set("test1".to_string(), json!("Hello"), 60);
	cache.set("test2".to_string(), json!(64), 60);
	cache.set("test3".to_string(), json!(3.43453), 60);
	cache.set("test4".to_string(), json!(true), 60);
	cache.set("test5".to_string(), json!(null), 60);
	cache.set("test6".to_string(), json!([1,2,3,4,5]), 60);
	cache.set("test7".to_string(), json!({ "theme": "dark", "refresh": 5 }), 60);

	cache.delete("test2");

	println!("{}", &cache.get("test4").unwrap().value);
	println!("{}", &cache.get("test5").unwrap().value);
	println!("{}", &cache.get("test6").unwrap().value);
	println!("{}", &cache.get("test7").unwrap().value);

	println!("{:?}", cache.list(1000, 0, "test"));
}