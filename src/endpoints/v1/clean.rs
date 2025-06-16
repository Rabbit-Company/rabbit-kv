use axum::body::Body;
use axum::http::Response;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};

use crate::caches::cache::Cache;
use crate::error::{Error, ErrorCode};
use crate::SharedState;

pub fn handle_ws(state: Arc<SharedState>) -> serde_json::Value {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.clean();

	serde_json::to_value(Error::from_code(ErrorCode::Success)).unwrap()
}

pub fn handle(state: Arc<SharedState>) -> Response<Body> {
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();
	shared_cache.clean();

	Json(Error::from_code(ErrorCode::Success)).into_response()
}

pub async fn handle_get(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
	if state.token.ne(bearer_token.token()) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
	}

	handle(state)
}
