use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use serde_json::Value;
use std::sync::{Arc,MutexGuard};
use serde::{Serialize, Deserialize};

use crate::utils::current_time;
use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::{Cache,CacheItem};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub key: String
}

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string() }).into_response();
  }

	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().unwrap();

	match shared_cache.get(&key) {
		Some(item) => {
			Json(CacheItem{ value: item.value.clone(), expiration: (item.expiration - current_time())/1000}).into_response()
		}
		None => {
			Json(Value::Null).into_response()
		}
	}
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

	match shared_cache.get(&payload.key) {
		Some(item) => {
			Json(CacheItem{ value: item.value.clone(), expiration: (item.expiration - current_time())/1000}).into_response()
		}
		None => {
			Json(Value::Null).into_response()
		}
	}
}