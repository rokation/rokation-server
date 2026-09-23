use serde::{Deserialize, Serialize};

use crate::{
    component::{Position, Velocity, lla::Lla},
    entity::{EntityId, EntityKind},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    SpawnEntity {
        entity_kind: EntityKind,
    },
    DestroyEntity {
        entity_id: EntityId,
    },
    SetLla {
        entity_id: EntityId,
        lla: Lla,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub success: bool,
    pub result: CommandResult,
}
