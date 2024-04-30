use crate::caches::cache::Cache;

pub struct SharedState {
	pub token: String,
	pub cache: Cache
}