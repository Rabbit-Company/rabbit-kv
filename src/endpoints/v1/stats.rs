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

  if state.token.clone().ne(bearer_token.token()) {
		return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
  }

	let shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	Json(&shared_cache.stats).into_response()
}