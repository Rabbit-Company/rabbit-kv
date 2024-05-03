use axum::{body::Body, http::Response, response::IntoResponse, Json};

use crate::error::{Error, ErrorCode};

pub fn handle() -> Response<Body>{
	Json(Error::from_code(ErrorCode::Success)).into_response()
}

pub async fn handle_get() -> impl IntoResponse{
	handle()
}