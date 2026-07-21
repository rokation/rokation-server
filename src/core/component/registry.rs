use std::{any::TypeId, collections::HashMap};

use crate::core::{
    component::{component::Component, store::ComponentStore},
    entity::entity::EntityId,
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

    pub fn storage<T: Component>(&mut self) -> &mut Storage<T> {
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

    pub fn remove_entity(&mut self, id: EntityId) {
        for store in self.stores.values_mut() {
            store.remove_entity(id);
        }
    }
}
