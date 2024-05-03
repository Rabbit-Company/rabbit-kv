use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc,MutexGuard};
use serde::{Serialize, Deserialize};

use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::Cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub prefix: String,
	pub limit: usize,
	pub cursor: usize,
}

pub async fn handle_get(
	Path((prefix, limit, cursor)): Path<(String, usize, usize)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string() }).into_response();
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	Json(shared_cache.list(limit, cursor, &prefix)).into_response()
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<Payload>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string() }).into_response();
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	Json(shared_cache.list(payload.limit, payload.cursor, &payload.prefix)).into_response()
}