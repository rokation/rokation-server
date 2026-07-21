use crate::core::entity::entity_id::EntityId;

#[derive(Debug, Default)]
pub struct EntityIdGenerator {
    next: u64,
}

impl EntityIdGenerator {
    pub fn next(&mut self) -> EntityId {
        let id = EntityId::new(self.next);
        self.next += 1;
        id
    }
}
