use std::any::TypeId;

use crate::entity::entity::EntityId;

#[derive(Debug)]
pub enum Event {
    EntityCreated(EntityId),
    EntityRemoved(EntityId),
    ComponentAdded { id: EntityId, component: TypeId },
    ComponentUpdated { id: EntityId, component: TypeId },
    ComponentRemoved { id: EntityId, component: TypeId },
}
