use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

use crate::error::ErrorCode;
use crate::state::SharedState;
use crate::types::{Actions, DataPayload, KeyPayload, ListPayload, NumberDataPayload};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub id: u64,
	pub action: Actions,
	pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpResponse {
	pub id: u64,
	pub code: u64,
	pub data: Option<serde_json::Value>,
}

pub async fn handle_client(stream: &mut TcpStream, state: Arc<SharedState>) {
	let mut buffer: [u8; 512] = [0; 512];
	loop {
		match stream.read(&mut buffer).await {
			Ok(n) => {
				if n == 0 {
					break;
				}
				if let Ok(payload) = serde_json::from_slice::<Payload>(&buffer[..n]) {
					let response = match payload.action {
						Actions::PING => super::endpoints::v1::ping::handle_ws(),
						Actions::STATS => super::endpoints::v1::stats::handle_ws(state.clone()),
						Actions::SAVE => super::endpoints::v1::save::handle_ws(state.clone()),
						Actions::CLEAN => super::endpoints::v1::clean::handle_ws(state.clone()),
						Actions::FLUSH => super::endpoints::v1::flush::handle_ws(state.clone()),
						Actions::GET => {
							if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
								super::endpoints::v1::get::handle_ws(state.clone(), data.key)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::SET => {
							if let Ok(data) = serde_json::from_value::<DataPayload>(payload.data) {
								super::endpoints::v1::set::handle_ws(state.clone(), data.key, data.value, data.ttl)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::DEL => {
							if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
								super::endpoints::v1::del::handle_ws(state.clone(), data.key)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::LIST => {
							if let Ok(data) = serde_json::from_value::<ListPayload>(payload.data) {
								super::endpoints::v1::list::handle_ws(state.clone(), data.prefix, data.limit, data.cursor)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::EXISTS => {
							if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
								super::endpoints::v1::exists::handle_ws(state.clone(), data.key)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::INCR => {
							if let Ok(data) = serde_json::from_value::<NumberDataPayload>(payload.data) {
								super::endpoints::v1::incr::handle_ws(state.clone(), data.key, data.value, data.ttl)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						},
						Actions::DECR => {
							if let Ok(data) = serde_json::from_value::<NumberDataPayload>(payload.data) {
								super::endpoints::v1::decr::handle_ws(state.clone(), data.key, data.value, data.ttl)
							}else{
								serde_json::to_value(TcpResponse{
									id: payload.id,
									code: ErrorCode::InvalidData as u64,
									data: None
								}).unwrap()
							}
						}
					};
					let response_json: String = serde_json::to_string(&response).unwrap();
					stream.write_all(response_json.as_bytes()).await.unwrap();
				} else {
					println!("Received invalid JSON data");
				}
			}
			Err(e) => {
				println!("Error reading from socket: {}", e);
				break;
			}
		}
	}
}

pub async fn authenticate(stream: &mut TcpStream, token: &str) -> bool {
	let mut buffer: Vec<u8> = vec![0; token.len()];
	match stream.read_exact(&mut buffer).await {
		Ok(_) => {
			if let Ok(received_token) = std::str::from_utf8(&buffer) {
				if received_token.trim() == token {
					// Authentication successful
					if let Err(e) = stream.write_all(b"Authenticated").await {
						println!("Error writing to socket: {}", e);
					}
					return true;
				}
			}
		}
		Err(e) => {
			println!("Error reading from socket: {}", e);
		}
	}
	// Authentication failed
	if let Err(e) = stream.write_all(b"Unauthorized").await {
		println!("Error writing to socket: {}", e);
	}
	false
}