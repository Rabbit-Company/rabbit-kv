use axum::{body::Body, http::Response, response::IntoResponse, Json};

use crate::error::Error;

pub fn handle() -> Response<Body>{
	Json(Error{ code: 0, message: "success".to_string() }).into_response()
}

pub async fn handle_get() -> impl IntoResponse{
	handle()
}