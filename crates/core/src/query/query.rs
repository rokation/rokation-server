use crate::{component::component::Component, storage::storage::Storage};

pub struct Query<'a, T: Component> {
    storage: Option<&'a Storage<T>>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(storage: Option<&'a Storage<T>>) -> Self {
        Self { storage }
    }
}
