use axum::body::Body;
use axum::http::Response;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use std::sync::Arc;

use crate::SharedState;
use crate::error::{Error, ErrorCode};

pub fn handle_ws() -> serde_json::Value{
	serde_json::to_value(Error::from_code(ErrorCode::Success)).unwrap()
}

pub fn handle() -> Response<Body>{
	Json(Error::from_code(ErrorCode::Success)).into_response()
}

pub async fn handle_get(
	State(state): State<Arc<SharedState>>,
	TypedHeader(bearer_token): TypedHeader<Authorization<Bearer>>
) -> impl IntoResponse{

  if state.token.ne(bearer_token.token()) {
    return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	handle()
}