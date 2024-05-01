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

async fn handle_socket(socket: WebSocket) {

	let (_, mut receiver) = socket.split();

	tokio::spawn(async move {
		let mut cnt = 0;
		while let Some(Ok(msg)) = receiver.next().await {
			cnt += 1;
			if process_message(msg).is_break() {
				break;
			}
		}
		cnt
	});

}

fn process_message(msg: Message) -> ControlFlow<(), ()> {
	match msg {

		Message::Text(t) => {
			println!(">>> received str: {t:?}");
		}

		Message::Binary(d) => {
			println!(">>> received {} bytes: {:?}", d.len(), d);
		}

		Message::Close(_) => { return ControlFlow::Break(()); }
		Message::Ping(_) => {}
		Message::Pong(_) => {}
	}
	ControlFlow::Continue(())
}