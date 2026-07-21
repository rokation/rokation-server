pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(u64);

impl EntityId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Default)]
pub struct EntityIdGenerator {
    next: u64,
}

impl EntityIdGenerator {
    pub fn next(&mut self) -> EntityId {
        let id = EntityId::new(self.next);
        self.next += 1;
        id
    }
}
