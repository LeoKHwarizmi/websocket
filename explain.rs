// Bring Axum's WebSocket types and routing tools into scope.
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade}, // WebSocket types
    response::IntoResponse,                              // For returning responses
    routing::get,                                        // For GET routes
    Router,                                              // The Axum router
};

// Standard library type for IP:PORT parsing.
use std::net::SocketAddr;

// The Tokio async runtime entry point.
// All async Rust servers start here.
#[tokio::main]
async fn main() {
    // Create the Axum router.
    // This router has ONLY ONE route: "/ws".
    // When someone visits /ws, we run ws_handler().
    let app = Router::new()
        .route("/ws", get(ws_handler));

    // Render sets PORT automatically.
    // Locally, we default to 10000.
    let port = std::env::var("PORT").unwrap_or("10000".into());

    // Convert "0.0.0.0:PORT" into a SocketAddr.
    // 0.0.0.0 means "listen on all network interfaces".
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Listening on {}", addr);

    // Start the Axum server.
    // axum::serve() takes a TCP listener and the router.
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}

// This function handles the HTTP → WebSocket upgrade.
// When a browser or Python client connects to /ws,
// Axum upgrades the connection to a WebSocket.
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    // After upgrading, Axum calls handle_socket().
    ws.on_upgrade(handle_socket)
}

// This function runs AFTER the WebSocket connection is established.
// It receives a WebSocket object that can send/receive messages.
async fn handle_socket(mut socket: WebSocket) {
    // Immediately send "Hello World" to the client.
    let _ = socket.send(Message::Text("Hello World".into())).await;

    // Loop forever, receiving messages from the client.
    while let Some(Ok(msg)) = socket.recv().await {
        // Only handle text messages.
        if let Message::Text(text) = msg {
            // Echo the message back to the client.
            let _ = socket.send(Message::Text(format!("You said: {}", text))).await;
        }
    }
}
