use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use serde_json::Value;
use std::sync::{Arc,MutexGuard};

use crate::utils::current_time;
use crate::SharedState;
use crate::error::Error;
use crate::caches::cache::{Cache,CacheItem};

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