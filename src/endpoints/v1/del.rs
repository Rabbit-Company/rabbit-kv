use axum::body::Body;
use axum::http::Response;
use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::types::KeyPayload;
use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::Cache;

pub fn handle(state: Arc<SharedState>, key: String) -> Response<Body>{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.delete(&key);

	Json(Error{ code: 0, message: "success".to_string() }).into_response()
}

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
  }

	handle(state, key)
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<KeyPayload>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string() }).into_response();
  }

	handle(state, payload.key)
}