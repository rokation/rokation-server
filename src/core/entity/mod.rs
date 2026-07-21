pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self { id }
    }
}

pub struct EntityId(u64);

impl EntityId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}
