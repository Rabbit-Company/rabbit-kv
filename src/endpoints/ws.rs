use axum::{body::{to_bytes, Body}, extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State}, http::{Response, StatusCode}, response::IntoResponse, Json};
use std::sync::Arc;
use std::ops::ControlFlow;
use futures::stream::StreamExt;
use serde::{Serialize, Deserialize};

use crate::{error::ErrorCode, types::{Actions, DataPayload, KeyPayload, ListPayload, NumberDataPayload}, SharedState};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub id: u64,
	pub action: Actions,
	pub data: serde_json::Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsResponse {
	pub id: u64,
	pub code: u64,
	pub data: Option<serde_json::Value>
}

pub async fn handle_get(
	ws: WebSocketUpgrade,
	State(state): State<Arc<SharedState>>,
	Path(token): Path<String>
) -> impl IntoResponse{

  if state.token.ne(&token) {
		return Json(Error::from_code(ErrorCode::InvalidToken)).into_response();
  }

	ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<SharedState>) {

	tokio::spawn(async move {
		let mut cnt = 0;
		while let Some(Ok(msg)) = socket.next().await {
			cnt += 1;
			if process_message(&mut socket, msg, state.clone()).await.is_break() {
				break;
			}
		}
		cnt
	});

}

async fn process_message(socket: &mut WebSocket, msg: Message, state: Arc<SharedState>) -> ControlFlow<(), ()> {
	match msg {
		Message::Text(t) => {
			if let Ok(payload) = serde_json::from_str::<Payload>(&t) {
				let res: Response<Body> = match payload.action {
					Actions::PING => super::v1::ping::handle(),
					Actions::STATS => super::v1::stats::handle(state),
					Actions::GET => {
						if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
							super::v1::get::handle(state, data.key)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::SET => {
						if let Ok(data) = serde_json::from_value::<DataPayload>(payload.data) {
							super::v1::set::handle(state, data.key, data.value, data.ttl)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::DEL => {
						if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
							super::v1::del::handle(state, data.key)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::LIST => {
						if let Ok(data) = serde_json::from_value::<ListPayload>(payload.data) {
							super::v1::list::handle(state, data.prefix, data.limit, data.cursor)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::EXISTS => {
						if let Ok(data) = serde_json::from_value::<KeyPayload>(payload.data) {
							super::v1::exists::handle(state, data.key)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::INCR => {
						if let Ok(data) = serde_json::from_value::<NumberDataPayload>(payload.data) {
							super::v1::incr::handle(state, data.key, data.value, data.ttl)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					},
					Actions::DECR => {
						if let Ok(data) = serde_json::from_value::<NumberDataPayload>(payload.data) {
							super::v1::decr::handle(state, data.key, data.value, data.ttl)
						}else{
							Json(WsResponse{
								id: payload.id,
								code: ErrorCode::InvalidData as u64,
								data: None
							}).into_response()
						}
					}
				};

				let json: serde_json::Value = response_to_json(res).await;

				let mut data: WsResponse = WsResponse{
					id: payload.id,
					code: 0,
					data: None
				};

				if let Some(code) = json.get("code").and_then(serde_json::Value::as_u64) {
					data.code = code;
				} else {
					data.data = Some(json);
				}

				socket.send(Message::Text(serde_json::to_string(&data).unwrap())).await.ok();
			}else{
				socket.send(Message::Text(serde_json::to_string(&Error::from_code(ErrorCode::InvalidPayload)).unwrap())).await.ok();
			}
		}

		Message::Binary(d) => {
			socket.send(Message::Binary(d)).await.ok();
		}

		Message::Close(_) => { return ControlFlow::Break(()); }
		Message::Ping(_) => {}
		Message::Pong(_) => {}
	}
	ControlFlow::Continue(())
}

async fn response_to_json(response: Response<Body>) -> serde_json::Value {
	if response.status() == StatusCode::OK {
		let body = response.into_body();
		let full_body = to_bytes(body, usize::MAX).await.unwrap();
		let body_str = String::from_utf8(full_body.to_vec()).unwrap();

		serde_json::from_str(&body_str).unwrap_or(serde_json::Value::Null)
	}else{
		serde_json::Value::Null
	}
}