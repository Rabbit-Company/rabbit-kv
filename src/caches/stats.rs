use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Stats {
	pub writes: u64,
	pub reads: u64,
	pub deletes: u64,
	pub lists: u64,
}
