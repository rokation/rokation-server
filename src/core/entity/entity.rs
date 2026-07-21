use uuid::Uuid;

pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            id: EntityId::new(),
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
