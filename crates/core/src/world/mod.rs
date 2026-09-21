use std::{collections::HashMap, io::ErrorKind};

use crate::{
    command::Command,
    component::{Position, Velocity, lla::Lla},
    entity::{Entity, EntityId, EntityKind, EntitySnapshot},
    event::Event::{self, EntityDestroyed, EntityMoved, EntitySpawned},
};
use uuid::Uuid;

pub type WorldId = Uuid;

#[derive(Clone)]
pub struct World {
    pub id: WorldId,
    pub origin: Lla,
    pub entities: HashMap<EntityId, Entity>,
    pub external_ids: HashMap<String, EntityId>,
    pub events: Vec<Event>,
}

impl World {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            origin: Lla::default(),
            entities: HashMap::new(),
            external_ids: HashMap::new(),
            events: Vec::new(),
        }
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn origin(&self) -> Lla {
        self.origin
    }

    pub fn spawn(&mut self, kind: EntityKind) -> EntityId {
        let entity = Entity::new(kind);
        let id = entity.id();
        let snapshot = entity.snapshot();

        self.entities.insert(id, entity);
        self.events.push(EntitySpawned(snapshot));
        id
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        if self.entities.remove(&entity_id).is_some() {
            self.external_ids.retain(|_, id| *id != entity_id);
            self.events.push(EntityDestroyed(entity_id));
        }
    }

    pub fn get(&self, entity_id: EntityId) -> Option<&Entity> {
        self.entities.get(&entity_id)
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&entity_id)
    }

    pub fn entities(&mut self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    pub fn entities_count(&self) -> usize {
        self.entities.len()
    }

    pub fn set_origin(&mut self, origin: Lla) {
        self.origin = origin;
    }

    pub fn set_lla(&mut self, entity_id: EntityId, lla: Lla) -> bool {
        if let Some(entity) = self.get_mut(entity_id) {
            entity.set_lla(lla);
            true
        } else {
            false
        }
    }

    pub fn set_position(&mut self, entity_id: EntityId, position: Position) -> bool {
        if let Some(entity) = self.get_mut(entity_id) {
            entity.position = position;
            true
        } else {
            false
        }
    }

    pub fn set_velocity(&mut self, entity_id: EntityId, velocity: Velocity) -> bool {
        if let Some(entity) = self.get_mut(entity_id) {
            entity.velocity = velocity;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut events = Vec::new();
        for entity in self.entities_mut() {
            let previous_position = entity.position();

            entity.update(dt);

            if entity.position() != previous_position {
                events.push(EntityMoved(entity.snapshot()));
            }
        }

        self.events.extend(events);
    }

    pub fn contains(&self, entity_id: EntityId) -> bool {
        self.entities.contains_key(&entity_id)
    }

    pub fn consume_event(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    pub fn snapshot(&self, entity_id: EntityId) -> Option<EntitySnapshot> {
        self.entities
            .get(&entity_id)
            .map(|entity| entity.snapshot())
    }

    pub fn snapshots(&self) -> Vec<EntitySnapshot> {
        self.entities
            .values()
            .map(|entity| entity.snapshot())
            .collect()
    }

    pub fn lla(&self, entity_id: EntityId) -> Option<Lla> {
        self.entities.get(&entity_id).map(|entity| entity.lla())
    }

    pub fn position(&self, entity_id: EntityId) -> Option<Position> {
        self.entities
            .get(&entity_id)
            .map(|entity| entity.position())
    }

    pub fn velocity(&self, entity_id: EntityId) -> Option<Velocity> {
        self.entities
            .get(&entity_id)
            .map(|entity| entity.velocity())
    }

    pub fn find_by_kind(&self, entity_kind: EntityKind) -> Vec<EntitySnapshot> {
        self.entities
            .values()
            .filter(|entity| entity.kind() == entity_kind)
            .map(|entity| entity.snapshot())
            .collect()
    }

    pub fn attach(&mut self, entity_id: EntityId, external_id: String) -> bool {
        if self.entities.contains_key(&entity_id) {
            return false;
        }

        if self.external_ids.contains_key(&external_id) {
            return false;
        }

        self.external_ids.insert(external_id, entity_id);
        true
    }

    pub fn find_by_external_id(&self, external_id: String) -> Option<EntitySnapshot> {
        self.external_ids
            .get(&external_id)
            .and_then(|entity_id| self.snapshot(*entity_id))
    }

    pub fn detach(&mut self, external_id: &str) -> bool {
        self.external_ids.remove(external_id).is_some()
    }

    pub fn external_id(&self, entity_id: EntityId) -> Option<&str> {
        self.external_ids
            .iter()
            .find(|(_, id)| entity_id == **id)
            .map(|(id, _)| id.as_str())
    }

    pub fn execute(&mut self, command: Command) -> Option<EntityId> {
        match command {
            Command::SpawnEntity { kind } => Some(self.spawn(kind)),
            Command::DestroyEntity { entity_id } => {
                self.destroy(entity_id);
                None
            }
            Command::SetPosition {
                entity_id,
                position,
            } => {
                self.set_position(entity_id, position);
                None
            }
            Command::SetVelocity {
                entity_id,
                velocity,
            } => {
                self.set_velocity(entity_id, velocity);
                None
            }
        }
    }
}
