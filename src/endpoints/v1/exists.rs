use axum::body::Body;
use axum::http::Response;
use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::caches::cache::Cache;
use crate::error::{Error, ErrorCode};
use crate::types::KeyPayload;
use crate::SharedState;

pub fn handle_ws(state: Arc<SharedState>, key: String) -> serde_json::Value {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	match shared_cache.get(&key) {
		Some(_) => serde_json::to_value(true).unwrap(),
		None => serde_json::to_value(false).unwrap(),
	}
}

pub fn handle(state: Arc<SharedState>, key: String) -> Response<Body> {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	match shared_cache.get(&key) {
		Some(_) => Json(true).into_response(),
		None => Json(false).into_response(),
	}
}

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
	}

	handle(state, key)
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<KeyPayload>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
	}

	handle(state, payload.key)
}
