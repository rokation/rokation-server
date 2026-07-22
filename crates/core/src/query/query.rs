use std::marker::PhantomData;

use crate::{
    component::{component::Component, registry::ComponentRegistry},
    entity::entity::EntityId,
    query::iterator::QueryIter,
};

pub struct Query<'a, Q> {
    registry: &'a ComponentRegistry,
    marker: PhantomData<Q>,
}

impl<'a, Q> Query<'a, Q> {
    pub fn new(registry: &'a ComponentRegistry) -> Self {
        Self {
            registry,
            marker: PhantomData,
        }
    }
}

impl<'a, T: Component> IntoIterator for Query<'a, T> {
    type Item = (&'a EntityId, &'a T);
    type IntoIter = QueryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter::new(self.registry.storage::<T>())
    }
}
