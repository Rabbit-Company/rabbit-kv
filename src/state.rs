use crate::caches::cache::Cache;
use tokio::sync::Mutex;

pub struct SharedState {
	pub token: String,
	pub cache: Mutex<Cache>
}