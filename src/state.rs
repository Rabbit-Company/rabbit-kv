use crate::caches::cache::Cache;
use std::sync::Mutex;

pub struct SharedState {
	pub token: String,
	pub cache: Mutex<Cache>
}