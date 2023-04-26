use axum::extract::State;
use std::sync::{Arc, Mutex};
use axum::Json;

use crate::{accounts::Accounts, errors::JValue};

pub async fn create_account(State(state): State<Arc<Mutex<Accounts>>>) -> Json<JValue>{
	Json(state.lock().unwrap().create("ziga.zajc007@gmail.com".to_string(), "".to_string(), "".to_string()).json(None))
}