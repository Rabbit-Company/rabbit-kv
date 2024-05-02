use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, MutexGuard};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::utils::current_time;
use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::Cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub key: String,
	pub value: i64,
	pub ttl: u64
}

pub async fn handle_get(
	Path((key, value, ttl)): Path<(String, i64, u64)>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	let new_value: i64 = match shared_cache.get(&key) {
		Some(item) => {
			if let Value::Number(n) = &item.value {
				if let Some(i) = n.as_i64() {
					match i.checked_sub(value) {
						Some(result) => result,
						None => {
							return Json(Error { code: 1004, message: "Integer overflow occurred".to_string() }).into_response();
						}
					}
				} else {
					return Json(Error { code: 1001, message: "Value is not an integer".to_string() }).into_response();
				}
			} else {
				return Json(Error { code: 1002, message: "Value is not a number".to_string() }).into_response();
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

	let new_value: i64 = match shared_cache.get(&payload.key) {
		Some(item) => {
			if let Value::Number(n) = &item.value {
				if let Some(i) = n.as_i64() {
					match i.checked_sub(payload.value) {
						Some(result) => result,
						None => {
							return Json(Error { code: 1004, message: "Integer overflow occurred".to_string() }).into_response();
						}
					}
				} else {
					return Json(Error { code: 1001, message: "Value is not an integer".to_string() }).into_response();
				}
			} else {
				return Json(Error { code: 1002, message: "Value is not a number".to_string() }).into_response();
			}
		},
		None => 1,
	};

	let new_ttl: u128 = match shared_cache.get(&payload.key) {
		Some(item) => {
			let current_time: u128 = current_time();
			if item.expiration > current_time {
				item.expiration - current_time
			} else {
				0
			}
		},
		None => 1000 * payload.ttl as u128,
	};

	shared_cache.set(payload.key.clone(), Value::Number(new_value.into()), new_ttl);

	Json(Error{ code: 0, message: "success".to_string() }).into_response()
}