use axum::{response::IntoResponse, Json};

pub async fn handle_get() -> impl IntoResponse{
	Json(serde_json::json!({"status": "success"}))
}