use crate::core::{
    component::{component::Component, registry::ComponentRegistry},
    entity::entity::{Entity, EntityId},
    error::{CoreError, Result},
    storage::storage::Storage,
};

pub struct World {
    entities: Storage<Entity>,
    components: ComponentRegistry,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Storage::<Entity>::new(),
            components: ComponentRegistry::new(),
        }
    }

    pub fn insert_component<T: Component>(&mut self, id: EntityId, component: T) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        self.components.storage::<T>().insert(id, component);

        Ok(())
    }

    pub fn get_component<T: Component>(&mut self, id: EntityId) -> Option<&T> {
        self.components.storage::<T>().get(id)
    }

    pub fn remove_component<T: Component>(&mut self, id: EntityId) {
        self.components.storage::<T>().remove(id)
    }

    pub fn spawn(&mut self) -> EntityId {
        let entity = Entity::new();
        let id = entity.id();
        self.entities.insert(entity.id(), entity);

        id
    }

    pub fn despawn(&mut self, id: EntityId) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        self.components.remove_entity(id);
        self.entities.remove(id);

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }
}
