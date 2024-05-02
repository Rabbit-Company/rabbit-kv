use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};
use serde::{Serialize, Deserialize};

use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::Cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub key: String,
	pub value: serde_json::Value,
	pub ttl: u64,
}

pub async fn handle_get(
	Path((key, value, ttl)): Path<(String, serde_json::Value, u64)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.set(key, value, 1000 * ttl as u128);

	Json(Error{ code: 0, message: "success".to_string() }).into_response()
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
	shared_cache.set(payload.key, payload.value, payload.ttl as u128 * 1000);

	Json(Error{ code: 0, message: "success".to_string() }).into_response()
}