use std::{collections::HashMap, sync::Arc};

use crate::{
    entity::{Entity, EntityId, EntityKind},
    geometry::{mbr::Mbr, vector::Point3},
    spatial::rtree::RTree,
};
use uuid::Uuid;

pub type WorldId = Uuid;

#[derive(Clone)]
pub struct World {
    pub id: WorldId,
    pub entities: HashMap<EntityId, Entity>,
    pub events: Vec<Event>,
}

impl World {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            spatial_index: RTree::new(),
            entities: HashMap::new(),
        }
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn spawn(&mut self, kind: EntityKind) -> EntityId {
        let entity = Entity::new(kind);
        let id = entity.id();

        let mbr = Mbr::from_point(entity.transform.position);

        self.insert(entity);
        self.spatial_index.insert(id, mbr);
        entity.id
    }

    pub fn despawn(&mut self, entity_id: EntityId) {
        self.entities.remove(&entity_id);
    }

    pub fn insert(&mut self, entity: Entity) {
        self.entities.insert(entity.id(), entity);
    }

    pub fn query_area(&self, query: &Mbr) -> Vec<EntityId> {
        self.spatial_index.search(&query)
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

    pub fn sync_spatial_index(&mut self) {
        let updates: Vec<(EntityId, Point3)> = self
            .entities
            .values()
            .map(|entity| (entity.id(), entity.transform.position))
            .collect();

        for (entity_id, position) in updates {
            self.spatial_index.remove(entity_id);
            self.spatial_index
                .insert(entity_id, Mbr::from_point(position));
        }
    }
}
