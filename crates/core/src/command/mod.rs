use serde::{Deserialize, Serialize};

use crate::{
    component::{Position, Velocity},
    entity::{EntityId, EntityKind},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    SpawnEntity {
        kind: EntityKind,
    },
    DestroyEntity {
        entity_id: EntityId,
    },
    SetPosition {
        entity_id: EntityId,
        position: Position,
    },
    SetVelocity {
        entity_id: EntityId,
        velocity: Velocity,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CommandResult {
    EntityCreated(EntityId),
    Success,
    NotFound,
}
