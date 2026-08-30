use serde::Serialize;

use crate::entity::EntityId;

#[derive(Debug, Serialize)]
pub struct Detection {
    pub sensor_id: EntityId,
    pub target_id: EntityId,
    pub distance: f64,
}
