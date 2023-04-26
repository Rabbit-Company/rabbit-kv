use axum::extract::State;
use std::sync::{Arc, Mutex};
use axum::Json;
use serde_json::Value;

use crate::{accounts::Accounts, errors::JValue};

pub async fn create_account(State(state): State<Arc<Mutex<Accounts>>>, Json(payload): Json<Value>) -> Json<JValue>{
	let username: &Value = &payload["username"];
	let password: &Value = &payload["password"];
	let email: &Value = &payload["email"];
	Json(state.lock().unwrap().create(username.to_string(), password.to_string(), email.to_string()).json(Some(payload)))
}