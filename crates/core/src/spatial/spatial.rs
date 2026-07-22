use std::collections::HashSet;

use crate::entity::entity::EntityId;

pub struct SpatialIndex {
    entities: HashSet<EntityId>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            entities: HashSet::new(),
        }
    }

    pub fn insert(&mut self, id: EntityId) {
        if !self.entities.contains(&id) {
            self.entities.insert(id);
        }
    }

    pub fn remove(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }
}
