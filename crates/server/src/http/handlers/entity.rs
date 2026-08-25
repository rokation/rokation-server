use std::time::Duration;

use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use rokation_core::{
    entity::{EntityId, EntityKind},
    world::WorldId,
};
use serde::Deserialize;
use tokio::time::interval;

use crate::AppState;

#[derive(Deserialize)]
pub struct EntityCreateRequest {
    pub world_id: WorldId,
    pub kind: EntityKind,
}

#[derive(Deserialize)]
pub struct EntityCreateResponse {
    pub entity_id: EntityId,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(world_id): Path<WorldId>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| read_entities(socket, state, world_id))
}

pub async fn read_entities(mut socket: WebSocket, state: AppState, world_id: WorldId) {
    loop {
        match socket.recv().await {
            Some(Ok(Message::Text(_))) => {
                if let Some(world) = { state.worlds.lock().await.get(&world_id).cloned() } {
                    let mut ticker = interval(Duration::from_millis(1000));
                    loop {
                        ticker.tick().await;
                        let data = serde_json::to_string(&world).expect("failed to serialize");
                        socket
                            .send(Message::Text(data.into()))
                            .await
                            .expect("failed to send");
                    }
                }
            }
            _ => {}
        }
    }
}
