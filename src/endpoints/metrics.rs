use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::Cache;

pub async fn handle_get(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
		return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
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
		 # EOF",
		shared_cache.stats.writes,
		shared_cache.stats.reads,
		shared_cache.stats.deletes,
		shared_cache.stats.lists
	);

	response_body.into_response()
}