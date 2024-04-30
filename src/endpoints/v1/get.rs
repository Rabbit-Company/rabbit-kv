use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc,MutexGuard};

use crate::SharedState;
use crate::caches::cache::Cache;
use crate::caches::cache::CacheItem;

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.clone().ne(bearer_token.token()) {
    return Json(serde_json::json!({ "status": 1000, "message": "Provided token is incorrect!"}));
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	let data: Option<&CacheItem> = shared_cache.get(&key);

	Json(serde_json::json!(data))
}