use std::{collections::HashMap, sync::Arc};

use serde::Serialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    component::position::Position,
    entity::{Entity, EntityId, EntityKind},
};

pub type WorldId = Uuid;

#[derive(Clone, Serialize)]
pub struct World {
    id: WorldId,
    entities: HashMap<EntityId, Entity>,
    positions: HashMap<EntityId, Position>,
}

impl World {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            entities: HashMap::new(),
            positions: HashMap::new(),
        }
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn spawn(&mut self, kind: EntityKind) -> EntityId {
        let e = Entity::new(kind);
        self.insert(e);
        e.id()
    }

    pub fn despawn(&mut self, entity_id: EntityId) {
        self.entities.remove(&entity_id);
    }

    pub fn insert(&mut self, entity: Entity) {
        self.entities.insert(entity.id(), entity);
    }

    pub fn get_entities(&self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.entities.iter()
    }

    pub fn get_mut_entities(&mut self) -> impl Iterator<Item = (&EntityId, &mut Entity)> {
        self.entities.iter_mut()
    }

    pub fn into_entities(self) -> impl Iterator<Item = (EntityId, Entity)> {
        self.entities.into_iter()
    }

    pub fn get_positions(&self) -> impl Iterator<Item = (&EntityId, &Position)> {
        self.positions.iter()
    }

    pub fn update_position(&mut self, entity_id: EntityId, position: Position) {
        self.positions.insert(entity_id, position);
    }

    pub fn get_position(&mut self, entity_id: EntityId) -> Option<&Position> {
        self.positions.get(&entity_id)
    }
}

pub struct WorldManager {
    worlds: HashMap<WorldId, World>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: HashMap::new(),
        }
    }

    pub async fn get(&self, world_id: WorldId) -> Option<&World> {
        self.worlds.get(&world_id)
    }

    pub async fn insert(&mut self, world: Arc<Mutex<World>>) {
        let w = world.lock().await;
        self.worlds.insert(w.id(), w.clone());
    }
}
