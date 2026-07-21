use std::any::Any;

use crate::core::entity::entity::EntityId;

pub trait ComponentStore {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_entity(&mut self, id: EntityId);
}
