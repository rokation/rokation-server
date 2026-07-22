use std::any::TypeId;

use crate::{
    component::{component::Component, registry::ComponentRegistry},
    entity::entity::{Entity, EntityId},
    error::{CoreError, Result},
    event::{event::Event, queue::EventQueue},
    foundation::position::Position,
    geometry::{bound::Bound, point::Point3},
    query::query::Query,
    spatial::spatial::SpatialIndex,
    storage::storage::Storage,
};

pub struct World {
    entities: Storage<Entity>,
    components: ComponentRegistry,
    events: EventQueue,
    spatial: SpatialIndex,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Storage::<Entity>::new(),
            components: ComponentRegistry::new(),
            events: EventQueue::new(),
            spatial: SpatialIndex::new(),
        }
    }

    pub fn insert<T: Component>(&mut self, id: EntityId, component: T) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        let added = self.components.insert(id, component);
        if added {
            self.events.push(Event::ComponentAdded {
                id,
                component: TypeId::of::<T>(),
            });
        } else {
            self.events.push(Event::ComponentUpdated {
                id,
                component: TypeId::of::<T>(),
            });
        }

        Ok(())
    }

    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T> {
        self.components.get::<T>(id)
    }

    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T> {
        self.components.get_mut::<T>(id)
    }

    pub fn remove<T: Component>(&mut self, id: EntityId) -> bool {
        self.components.remove::<T>(id)
    }

    pub fn spawn(&mut self) -> EntityId {
        let entity = Entity::new();
        let id = entity.id();

        self.entities.insert(entity.id(), entity);
        self.events.push(Event::EntityCreated(id));

        id
    }

    pub fn despawn<T: Component>(&mut self, id: EntityId) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        self.components.remove::<T>(id);
        self.entities.remove(id);
        self.events.push(Event::EntityRemoved(id));

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn drain_events(&mut self) -> Vec<Event> {
        self.events.drain()
    }

    pub fn query<Q>(&self) -> Query<'_, Q> {
        Query::new(&self.components)
    }

    pub fn position(&self, id: EntityId) -> Option<&Point3> {
        self.components.get::<Position>(id).map(|pos| &pos.point)
    }

    pub fn query_area(&self, bounds: &Bound) -> Vec<EntityId> {
        let mut result = Vec::new();
        for id in self.spatial.entities() {
            if let Some(pos) = self.position(*id) {
                if bounds.contains(pos) {
                    result.push(*id)
                }
            }
        }

        result
    }
}
