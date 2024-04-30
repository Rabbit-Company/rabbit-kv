use axum::{response::IntoResponse, Json};

use crate::error::Error;

pub async fn handle_get() -> impl IntoResponse{
	Json(Error{ code: 0, message: "success".to_string() }).into_response()
}