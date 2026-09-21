use crate::entity::{EntityId, EntitySnapshot};

#[derive(Debug, Clone)]
pub enum Event {
    EntitySpawned(EntitySnapshot),
    EntityMoved(EntitySnapshot),
    EntityDestroyed(EntityId),
}
