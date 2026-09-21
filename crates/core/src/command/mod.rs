use crate::{
    component::{Position, Velocity},
    entity::{EntityId, EntityKind},
};

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
