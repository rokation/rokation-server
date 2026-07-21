use crate::core::entity::entity_id::EntityId;

pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}
