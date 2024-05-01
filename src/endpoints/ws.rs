use axum::{extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State}, response::IntoResponse, Json};
use std::sync::Arc;
use std::ops::ControlFlow;
use futures::stream::StreamExt;

use crate::SharedState;
use crate::error::Error;

pub async fn handle_get(
	ws: WebSocketUpgrade,
	State(state): State<Arc<SharedState>>,
	Path(token): Path<String>
) -> impl IntoResponse{

  if state.token.ne(&token) {
		return Json(Error{ code: 1000, message: "Provided token is incorrect!".to_string()}).into_response();
  }

	ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {

	tokio::spawn(async move {
		let mut cnt = 0;
		while let Some(Ok(msg)) = socket.next().await {
			cnt += 1;
			if process_message(&mut socket, msg).await.is_break() {
				break;
			}
		}
		cnt
	});

}

async fn process_message(socket: &mut WebSocket, msg: Message) -> ControlFlow<(), ()> {
	match msg {

		Message::Text(t) => {
			socket.send(Message::Text(t)).await.ok();
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