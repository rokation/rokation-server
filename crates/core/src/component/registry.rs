use std::{any::TypeId, collections::HashMap};

use crate::{
    component::{component::Component, store::ComponentStore},
    entity::entity::EntityId,
    query::query::Query,
    storage::storage::Storage,
};

pub struct ComponentRegistry {
    stores: HashMap<TypeId, Box<dyn ComponentStore>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }

    fn storage_mut<T: Component>(&mut self) -> &mut Storage<T> {
        let id = TypeId::of::<T>();
        if !self.stores.contains_key(&id) {
            self.stores.insert(id, Box::new(Storage::<T>::new()));
        }

        self.stores
            .get_mut(&id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<Storage<T>>() // dyn Any안의 타입이 Storage<T>라면 반환
            .unwrap()
    }

    fn storage<T: Component>(&self) -> Option<&Storage<T>> {
        let id = TypeId::of::<T>();

        self.stores
            .get(&id)
            .unwrap()
            .as_any()
            .downcast_ref::<Storage<T>>()
    }

    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T> {
        self.storage::<T>()?.get(id)
    }

    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T> {
        self.storage_mut::<T>().get_mut(id)
    }

    pub fn insert<T: Component>(&mut self, id: EntityId, component: T) -> bool {
        self.storage_mut::<T>().insert(id, component)
    }

    pub fn remove<T: Component>(&mut self, id: EntityId) -> bool {
        let type_id = TypeId::of::<T>();
        let Some(store) = self.stores.get_mut(&type_id) else {
            return false;
        };

        store
            .as_any_mut()
            .downcast_mut::<Storage<T>>()
            .unwrap()
            .remove(id)
            .is_some()
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        for store in self.stores.values_mut() {
            store.remove_entity(entity);
        }
    }

    pub fn query<T: Component>(&self) -> Query<'_, T> {
        Query::new(self.storage::<T>())
    }
}
