use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::atomic::Ordering;
use std::sync::{Arc, MutexGuard};

use crate::caches::cache::Cache;
use crate::error::{Error, ErrorCode};
use crate::SharedState;

pub async fn handle_get(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::Success)).into_response();
	}

	let shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	let response_body: String = format!(
		"# HELP cache_writes Total cache writes\n\
		 # TYPE cache_writes counter\n\
		 cache_writes {}\n\
		 # HELP cache_reads Total cache reads\n\
		 # TYPE cache_reads counter\n\
		 cache_reads {}\n\
		 # HELP cache_deletes Total cache deletes\n\
		 # TYPE cache_deletes counter\n\
		 cache_deletes {}\n\
		 # HELP cache_lists Total cache lists\n\
		 # TYPE cache_lists counter\n\
		 cache_lists {}\n\
		 # HELP cache_keys Number of keys in a cache\n\
		 # TYPE cache_keys gauge\n\
		 cache_keys {}\n\
		 # HELP ws_connections Number of open WebSocket connections\n\
		 # TYPE ws_connections gauge\n\
		 ws_connections {}\n\
		 # EOF",
		shared_cache.stats.writes,
		shared_cache.stats.reads,
		shared_cache.stats.deletes,
		shared_cache.stats.lists,
		shared_cache.cache.len(),
		state.ws_connections.load(Ordering::Acquire)
	);

	response_body.into_response()
}
