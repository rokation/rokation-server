use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    component::{sensor::Sensor, transform::Transform},
    geometry::vector::{Point3, Vec3},
};

pub type EntityId = Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EntityKind {
    Drone,
    Robot,
    CCTV,
    Sensor,
    Vessel,
    Soldier,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub transform: Transform,
    pub velocity: Vec3,
    pub sensor: Option<Sensor>,
}

impl Entity {
    pub fn new(kind: EntityKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            transform: Transform {
                position: Point3::default(),
            },
            velocity: Vec3::default(),
            sensor: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
