use axum::{extract::State, extract::Path, body::Body, http::Response, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::MutexGuard;

use crate::types::KeyPayload;
use crate::utils::current_time;
use crate::SharedState;
use crate::error::{Error, ErrorCode};
use crate::caches::cache::{Cache,CacheItem};

pub async fn handle(state: Arc<SharedState>, key: String) -> Response<Body>{
	let mut shared_cache: MutexGuard<Cache> = state.cache.lock().await;

	match shared_cache.get(&key) {
		Some(item) => Json(CacheItem{
			value: item.value.clone(),
			expiration: (item.expiration - current_time())/1000
		}).into_response(),
		None => Json(Value::Null).into_response()
	}
}

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, key).await
}

pub async fn handle_post(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<KeyPayload>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle(state, payload.key).await
}