use axum::response::Json;
use serde_json::{Value, json};

pub async fn create_account() -> Json<Value>{
	Json(json!({"info": 2}))
}