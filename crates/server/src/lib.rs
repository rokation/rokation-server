use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    http::StatusCode,
    response::{ErrorResponse, IntoResponse},
};
use rokation_core::world::{World, WorldId};
use tokio::sync::Mutex;

pub mod http;

#[derive(Clone)]
pub struct AppState {
    pub worlds: Arc<Mutex<HashMap<WorldId, World>>>,
}

pub enum AppError {
    NotFound,
}
