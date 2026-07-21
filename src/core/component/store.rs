use std::any::Any;

use crate::core::{component::component::Component, storage::storage::Storage};

pub trait ComponentStore {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// eg. impl Component<Position> ComponentStore for Storage<Position>
impl<T: Component> ComponentStore for Storage<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
