use std::collections::HashMap;

use crate::core::entity::entity::EntityId;

pub struct Storage<T> {
    data: HashMap<EntityId, T>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: EntityId, value: T) {
        self.data.insert(id, value);
    }

    pub fn remove(&mut self, id: EntityId) {
        self.data.remove(&id);
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.data.get(&id)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
