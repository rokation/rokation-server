use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::core::storage::storage::{ComponentStore, Storage};

pub trait Component: Any {}

// eg. impl Component<Position> for Position
impl<T: Any> Component for T {}
