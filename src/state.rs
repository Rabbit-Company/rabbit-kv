use crate::caches::cache::Cache;
use std::sync::{atomic::AtomicU64, Mutex};

pub struct SharedState {
	pub token: String,
	pub cache: Mutex<Cache>,
	pub ws_connections: AtomicU64
}