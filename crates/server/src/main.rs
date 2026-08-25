use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use rokation_core::error::Result;
use rokation_server::{AppState, http::handlers::entity::websocket_handler};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState {
        worlds: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/worlds/{world_id}", get(websocket_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app).await.expect("server failed");
    Ok(())
}
