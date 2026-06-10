use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;

mod terminal;

#[tokio::main]
async fn main() {
let app = Router::new()
    .route("/", get(|| async { "OK" }))
    .route("/ws", get(ws_handler))
    .route("/terminal", get(terminal::terminal_handler));


    let port = std::env::var("PORT").unwrap_or("10000".into());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let _ = socket.send(Message::Text("Hello World".into())).await;

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let _ = socket.send(Message::Text(format!("You said: {}", text))).await;
        }
    }
}
