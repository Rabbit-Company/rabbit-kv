use axum::{extract::State, extract::Path, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::SharedState;

pub async fn handle_get(
	Path(key): Path<String>,
	State(state): State<Arc<Mutex<SharedState>>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{
  let mut shared_state: MutexGuard<'_, SharedState> = state.lock().unwrap();
  if shared_state.token.ne(bearer_token.token()) {
    return Json(serde_json::json!({ "status": 1000, "message": "Provided token is incorrect!"}));
  }

	let data = shared_state.cache.get(&key);

	Json(serde_json::json!(data))
}