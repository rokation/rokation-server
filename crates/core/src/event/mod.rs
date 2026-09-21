use serde::{Deserialize, Serialize};

use crate::entity::{EntityId, EntitySnapshot};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Event {
    EntitySpawned(EntitySnapshot),
    EntityMoved(EntitySnapshot),
    EntityDestroyed(EntityId),
}
