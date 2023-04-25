use crate::caches::cache::Cache;
use crate::caches::stats::Stats;
use crate::errors::Error;

use self::account::Account;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Result, BufReader, BufWriter};

pub mod stats;
pub mod account;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
	pub username: String,
	pub password: String,
	pub email: String,
	pub caches: HashMap<String, Stats>,
	pub created: u64,
	pub accessed: u64
}

pub struct Accounts {
	pub accounts: Vec<Account>
}

impl Accounts {

	pub fn new() -> Self {
		Accounts { accounts: vec![] }
	}

	pub fn import(&mut self) -> Result<()>{
		let file: File = File::open("/var/lib/rabbitkv/accounts.jsonl")?;
		let reader: BufReader<File> = BufReader::new(file);
		for line in reader.lines() {
			let kv: KeyValue = serde_json::from_str(line.as_ref().unwrap())?;

			let mut caches: HashMap<String, Cache> = HashMap::new();
			for (id, stats) in &kv.caches {
				let mut cache = Cache::new(id.clone());
				cache.stats = stats.clone();
				caches.insert(id.clone(), cache);
			}
			self.accounts.push( Account { username: kv.username, password: kv.password, email: kv.email, caches, created: kv.created, accessed: kv.accessed} );
		}
		Ok(())
	}

	pub fn save(&self) -> Result<()>{
		let file: File = OpenOptions::new().write(true).truncate(true).create(true).open("/var/lib/rabbitkv/accounts.jsonl")?;
		let mut writer: BufWriter<File> = BufWriter::new(file);
		for account in &self.accounts {
			let mut caches: HashMap<String, Stats> = HashMap::new();

			for (id, cache) in &account.caches {
				caches.insert(id.clone(), cache.stats.clone());
			}

			let kv: KeyValue = KeyValue { username: account.username.clone(), password: account.password.clone(), email: account.email.clone(), caches, created: account.created, accessed: account.accessed };
			let line: String = serde_json::to_string(&kv)?;
			writeln!(writer, "{}", line)?;
		}
		Ok(())
	}

	pub fn create(&mut self, username: String, password: String, email: String) -> Error{

		for account in &self.accounts {
			if account.username == username { return Error::UsernameExists }
		}

		self.accounts.push(Account::new(username, password, email));
		self.save().ok();

		Error::Success
	}

}

impl Default for Accounts {
	fn default() -> Self {
		Self::new()
	}
}