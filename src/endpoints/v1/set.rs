use axum::body::Body;
use axum::http::Response;
use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::caches::cache::Cache;
use crate::error::{Error, ErrorCode};
use crate::types::DataPayload;
use crate::SharedState;

pub fn handle_ws(
	state: Arc<SharedState>,
	key: String,
	value: serde_json::Value,
	ttl: u64,
) -> serde_json::Value {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.set(key, value, 1000 * ttl as u128);

	serde_json::to_value(Error::from_code(ErrorCode::Success)).unwrap()
}

pub fn handle(
	state: Arc<SharedState>,
	key: String,
	value: serde_json::Value,
	ttl: u64,
) -> Response<Body> {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.set(key, value, 1000 * ttl as u128);

	Json(Error::from_code(ErrorCode::Success)).into_response()
}

pub async fn handle_get(
	Path((key, value, ttl)): Path<(String, serde_json::Value, u64)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
	}

	Json(handle_ws(state, key, value, ttl)).into_response()
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<DataPayload>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
	}

	handle(state, payload.key, payload.value, payload.ttl)
}
