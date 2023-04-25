pub mod caches;
pub mod accounts;

use std::fs;
use serde_json::json;

use crate::caches::cache;

fn main() {

	fs::create_dir_all("/var/lib/rabbitkv/storage").expect("Permission denied. Please run program with root user.");
	let connection: sqlite::Connection = sqlite::open("/var/lib/rabbitkv/database.sqlite").unwrap();

	let query: &str = "
		CREATE TABLE IF NOT EXISTS accounts(
			username TEXT PRIMARY KEY,
			password TEXT NOT NULL,
			email TEXT NOT NULL,
			created TEXT NOT NULL,
			accessed TEXT NOT NULL
		);

		CREATE TABLE IF NOT EXISTS caches(
			id TEXT PRIMARY KEY,
			owner TEXT NOT NULL,
			writes INTEGER NOT NULL DEFAULT 0,
			reads INTEGER NOT NULL DEFAULT 0,
			deletes INTEGER NOT NULL DEFAULT 0,
			lists INTEGER NOT NULL DEFAULT 0
		);
	";

	connection.execute(query).unwrap();

	let mut cache: cache::Cache = cache::Cache::new();

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