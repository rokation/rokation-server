use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::component::{Position, Velocity, lla::Lla};

pub type EntityId = Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    pub lla: Lla,
    pub position: Position,
    pub velocity: Velocity,
}

impl Entity {
    pub fn new(kind: EntityKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            lla: Lla::default(),
            velocity: Velocity::default(),
            position: Position::default(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn kind(&self) -> EntityKind {
        self.kind
    }

    pub fn lla(&self) -> Lla {
        self.lla
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn set_lla(&mut self, lla: Lla) {
        self.lla = lla;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn update(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }

    pub fn snapshot(&self) -> EntitySnapshot {
        EntitySnapshot {
            entity_id: self.id,
            entity_kind: self.kind,
            lla: self.lla,
            position: self.position,
            velocity: self.velocity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub entity_id: EntityId,
    pub entity_kind: EntityKind,
    pub lla: Lla,
    pub position: Position,
    pub velocity: Velocity,
}
