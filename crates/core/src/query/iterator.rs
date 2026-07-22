use std::collections::hash_map::Iter;

use crate::{component::component::Component, entity::entity::EntityId, storage::storage::Storage};

pub struct QueryIter<'a, T: Component> {
    inner: Option<Iter<'a, EntityId, T>>, // 스토리지 없는 컴포넌트 존재
}

impl<'a, T: Component> QueryIter<'a, T> {
    pub fn new(storage: Option<&'a Storage<T>>) -> Self {
        Self {
            inner: storage.map(|storage| storage.iter()),
        }
    }
}

impl<'a, T: Component> Iterator for QueryIter<'a, T> {
    type Item = (&'a EntityId, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut()?.next()
    }
}
