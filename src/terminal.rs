use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::process::Command;

pub async fn terminal_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(run_terminal)
}

async fn run_terminal(mut socket: WebSocket) {
    let _ = socket.send(Message::Text("Terminal ready".into())).await;

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(cmd) = msg {
            if cmd.trim().is_empty() {
                continue;
            }
            let output = run_command(&cmd).await;
            let cleaned = output.trim().to_string();
            let _ = socket.send(Message::Text(cleaned)).await;
        }
    }
}

async fn run_command(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return "No command".into();
    }

    let program = parts[0];
    let args = &parts[1..];

    let result = Command::new(program)
        .args(args)
        .output()
        .await;

    match result {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);

            // ⭐ NEW: cleaner formatting
            if stderr.trim().is_empty() {
                stdout.to_string()
            } else {
                format!("{}\n{}", stdout, stderr)
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}
