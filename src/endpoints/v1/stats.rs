use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::SharedState;
use crate::caches::cache::Cache;

pub async fn handle_get(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.clone().ne(bearer_token.token()) {
    return Json(serde_json::json!({ "status": 1000, "message": "Provided token is incorrect!"}));
  }

	let shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	let data = shared_cache.stats.clone();

	Json(serde_json::json!(data))
}