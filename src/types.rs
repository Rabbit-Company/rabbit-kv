use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Actions {
	GET,
	SET,
	DEL,
	LIST,
	EXISTS,
	INCR,
	DECR,
	SAVE,
	CLEAN,
	FLUSH,
	PING,
	STATS
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPayload {
	pub key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataPayload {
	pub key: String,
	pub value: serde_json::Value,
	pub ttl: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPayload {
	pub prefix: String,
	pub limit: usize,
	pub cursor: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberDataPayload {
	pub key: String,
	pub value: i64,
	pub ttl: u64
}