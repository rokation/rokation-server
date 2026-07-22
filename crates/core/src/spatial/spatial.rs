use std::collections::HashSet;

use chrono::format::Item;

use crate::{entity::entity::EntityId, geometry::bound::Bound, world::world::World};

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

    pub fn query(&self, bounds: &Bound, world: &World) -> Vec<EntityId> {
        let mut result = Vec::new();

        for id in &self.entities {
            if let Some(position) = world.position(*id) {
                if bounds.contains(position) {
                    result.push(*id);
                }
            }
        }

        result
    }

    pub fn entities(&self) -> impl IntoIterator<Item = &EntityId> {
        self.entities.iter()
    }
}
