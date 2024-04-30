use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, Mutex, MutexGuard};
use serde::{Serialize, Deserialize};

use crate::SharedState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub key: String,
	pub value: serde_json::Value,
	pub ttl: u64,
}

pub async fn handle_get(
	Path((key, value, ttl)): Path<(String, serde_json::Value, u64)>,
	State(state): State<Arc<Mutex<SharedState>>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{
  let mut shared_state: MutexGuard<'_, SharedState> = state.lock().unwrap();
  if shared_state.token.ne(bearer_token.token()) {
    return Json(serde_json::json!({ "status": 1000, "message": "Provided token is incorrect!"}));
  }

	shared_state.cache.set(key, value, ttl);

	Json(serde_json::json!({"status": "success"}))
}

pub async fn handle_post(
	State(state): State<Arc<Mutex<SharedState>>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>,
	Json(payload): Json<Payload>
) -> impl IntoResponse{
  let mut shared_state: MutexGuard<'_, SharedState> = state.lock().unwrap();
  if shared_state.token.ne(bearer_token.token()) {
    return Json(serde_json::json!({ "status": 1000, "message": "Provided token is incorrect!"}));
  }

	shared_state.cache.set(payload.key, payload.value, payload.ttl);

	Json(serde_json::json!({"status": "success"}))
}