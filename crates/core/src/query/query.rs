use crate::{
    component::component::Component, entity::entity::EntityId, query::iterator::QueryIter,
    storage::storage::Storage,
};

pub struct Query<'a, T: Component> {
    storage: Option<&'a Storage<T>>,
}

impl<'a, T: Component> Query<'a, T> {
    pub fn new(storage: Option<&'a Storage<T>>) -> Self {
        Self { storage }
    }
}

impl<'a, T: Component> IntoIterator for Query<'a, T> {
    type Item = (&'a EntityId, &'a T);
    type IntoIter = QueryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter::new(self.storage)
    }
}
