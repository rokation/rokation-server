use uuid::Uuid;

pub type EntityId = Uuid;

#[derive(Debug, Clone, Copy)]
pub enum EntityKind {
    Drone,
    Robot,
    CCTV,
    Sensor,
    Vessel,
    Soldier,
}

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    id: EntityId,
    kind: EntityKind,
}

impl Entity {
    pub fn new(kind: EntityKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
