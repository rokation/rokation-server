use crate::core::entity::Entity;

pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}
