use axum::body::Body;
use axum::http::Response;
use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};
use serde_json::Value;

use crate::types::NumberDataPayload;
use crate::utils::current_time;
use crate::SharedState;
use crate::error::{Error, ErrorCode};
use crate::caches::cache::Cache;

pub fn handle_ws(state: Arc<SharedState>, key: String, value: i64, ttl: u64) -> serde_json::Value{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	let new_value: i64 = match shared_cache.get(&key) {
		Some(item) => {
			if let Value::Number(n) = &item.value {
				if let Some(i) = n.as_i64() {
					match i.checked_sub(value) {
						Some(result) => result,
						None => {
							return serde_json::to_value(Error::from_code(ErrorCode::IntegerOverflow)).unwrap();
						}
					}
				} else {
					return serde_json::to_value(Error::from_code(ErrorCode::InvalidInteger)).unwrap();
				}
			} else {
				return serde_json::to_value(Error::from_code(ErrorCode::InvalidNumber)).unwrap();
			}
		},
		None => 1,
	};

	let new_ttl: u128 = match shared_cache.get(&key) {
		Some(item) => {
			let current_time: u128 = current_time();
			if item.expiration > current_time {
				item.expiration - current_time
			} else {
				0
			}
		},
		None => 1000 * ttl as u128,
	};

	shared_cache.set(key.clone(), Value::Number(new_value.into()), new_ttl);

	serde_json::to_value(Error::from_code(ErrorCode::Success)).unwrap()
}

pub fn handle(state: Arc<SharedState>, key: String, value: i64, ttl: u64) -> Response<Body>{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	let new_value: i64 = match shared_cache.get(&key) {
		Some(item) => {
			if let Value::Number(n) = &item.value {
				if let Some(i) = n.as_i64() {
					match i.checked_sub(value) {
						Some(result) => result,
						None => {
							return Json(Error::from_code(ErrorCode::IntegerOverflow)).into_response();
						}
					}
				} else {
					return Json(Error::from_code(ErrorCode::InvalidInteger)).into_response();
				}
			} else {
				return Json(Error::from_code(ErrorCode::InvalidNumber)).into_response();
			}
		},
		None => 1,
	};

	let new_ttl: u128 = match shared_cache.get(&key) {
		Some(item) => {
			let current_time: u128 = current_time();
			if item.expiration > current_time {
				item.expiration - current_time
			} else {
				0
			}
		},
		None => 1000 * ttl as u128,
	};

	shared_cache.set(key.clone(), Value::Number(new_value.into()), new_ttl);

	Json(Error::from_code(ErrorCode::Success)).into_response()
}

pub async fn handle_get(
	Path((key, value, ttl)): Path<(String, i64, u64)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, key, value, ttl)
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<NumberDataPayload>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, payload.key, payload.value, payload.ttl)
}