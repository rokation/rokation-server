use std::{any::TypeId, time::Duration};

use crate::{
    component::{component::Component, registry::ComponentRegistry},
    entity::entity::{Entity, EntityId},
    error::{CoreError, Result},
    event::{event::Event, queue::EventQueue},
    foundation::{position::Position, velocity::Velocity},
    geometry::{bound::Bounds, point::Point3, vector::Vector3},
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
            spatial: SpatialIndex::new(100.0),
        }
    }

    pub fn set_position(&mut self, id: EntityId, position: Position) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        let new_point = position.point;

        let old_position = self.position(id).copied();
        let added = self.components.insert(id, position);

        match old_position {
            Some(old) => {
                self.spatial.move_entity(id, &old, &new_point);
            }
            None => self.spatial.insert(id, &new_point),
        }

        if added {
            self.events.push(Event::ComponentAdded {
                id,
                component: TypeId::of::<Position>(),
            });
        } else {
            self.events.push(Event::ComponentUpdated {
                id,
                component: TypeId::of::<Position>(),
            });
        }

        Ok(())
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

    pub fn despawn(&mut self, id: EntityId) -> Result<()> {
        if !self.entities.contains(id) {
            return Err(CoreError::EntityNotFound(id));
        }

        if let Some(position) = self.position(id).cloned() {
            self.spatial.remove(id, &position);
        }

        self.components.remove_entity(id);
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

    pub fn position(&self, id: EntityId) -> Option<&Point3> {
        self.components.get::<Position>(id).map(|pos| &pos.point)
    }

    pub fn query<Q>(&self) -> Query<'_, Q> {
        Query::new(&self.components)
    }

    pub fn query_area(&self, bounds: &Bounds) -> Vec<EntityId> {
        self.spatial.query(bounds, self)
    }

    pub fn translate(&mut self, id: EntityId, delta: Vector3) -> Result<()> {
        let current = self
            .components
            .get::<Position>(id)
            .ok_or(CoreError::EntityNotFound(id))?;

        let next = current.point + delta;
        self.set_position(id, Position::new(next))?;

        Ok(())
    }

    pub fn move_by_velocity(&mut self, id: EntityId, dt: Duration) -> Result<()> {
        let position = self
            .get::<Position>(id)
            .copied()
            .ok_or(CoreError::ComponentNotFound(TypeId::of::<Position>()))?;

        let velocity = self
            .get::<Velocity>(id)
            .copied()
            .ok_or(CoreError::ComponentNotFound(TypeId::of::<Velocity>()))?;

        let delta = velocity.linear * dt.as_secs_f64();
        let next = Position::new(position.point + delta);

        self.set_position(id, next)
    }

    pub fn update(&mut self, dt: Duration) -> Result<()> {
        let targets: Vec<EntityId> = self
            .query::<Velocity>()
            .into_iter()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        for id in targets {
            self.move_by_velocity(id, dt)?;
        }

        Ok(())
    }
}
