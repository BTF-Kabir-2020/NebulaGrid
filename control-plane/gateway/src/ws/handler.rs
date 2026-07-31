use super::hub::WsHub;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(hub): Extension<WsHub>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, hub))
}

async fn handle_socket(mut socket: WebSocket, hub: WsHub) {
    let mut rx = hub.subscribe("metrics").await;

    loop {
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if socket.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            }
            ws_msg = socket.recv() => {
                match ws_msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}
