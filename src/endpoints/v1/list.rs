use axum::body::Body;
use axum::http::Response;
use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::types::ListPayload;
use crate::SharedState;
use crate::error::{Error, ErrorCode};
use crate::caches::cache::Cache;

pub fn handle_ws(state: Arc<SharedState>, prefix: String, limit: usize, cursor: usize) -> serde_json::Value{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	serde_json::to_value(shared_cache.list(limit, cursor, &prefix)).unwrap()
}

pub fn handle(state: Arc<SharedState>, prefix: String, limit: usize, cursor: usize) -> Response<Body>{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	Json(shared_cache.list(limit, cursor, &prefix)).into_response()
}

pub async fn handle_get(
	Path((prefix, limit, cursor)): Path<(String, usize, usize)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, prefix, limit, cursor)
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<ListPayload>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, payload.prefix, payload.limit, payload.cursor)
}