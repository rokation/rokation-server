use std::{any::Any, collections::HashMap};

use crate::core::{
    component::{component::Component, store::ComponentStore},
    entity::entity::EntityId,
};

pub struct Storage<T> {
    data: HashMap<EntityId, T>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: EntityId, value: T) -> bool {
        self.data.insert(id, value).is_none()
    }

    pub fn remove(&mut self, id: EntityId) -> Option<T> {
        self.data.remove(&id)
    }

    pub fn get(&self, id: EntityId) -> Option<&T> {
        self.data.get(&id)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.data.contains_key(&id)
    }
}

// eg. impl Component<Position> ComponentStore for Storage<Position>
impl<T: Component> ComponentStore for Storage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_entity(&mut self, id: EntityId) {
        self.data.remove(&id);
    }
}
