use std::collections::{HashMap, HashSet};

use crate::{
    entity::entity::EntityId,
    geometry::{bound::Bounds, point::Point3},
    spatial::cell::Cell,
    world::world::World,
};

pub struct SpatialIndex {
    cell_size: f64,
    cells: HashMap<Cell, HashSet<EntityId>>,
}

impl SpatialIndex {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: EntityId, point: &Point3) {
        let cell = self.position_to_cell(point);

        self.cells
            .entry(cell)
            .or_insert_with(HashSet::new)
            .insert(id);
    }

    pub fn remove(&mut self, id: EntityId, point: &Point3) {
        let cell = self.position_to_cell(point);

        if let Some(entities) = self.cells.get_mut(&cell) {
            entities.remove(&id);
        }
    }

    fn position_to_cell(&self, point: &Point3) -> Cell {
        Cell {
            x: (point.x / self.cell_size).floor() as i32,
            y: (point.y / self.cell_size).floor() as i32,
        }
    }

    pub fn cells_in_bounds(&self, bounds: &Bounds) -> Vec<Cell> {
        let min = self.position_to_cell(&bounds.min);
        let max = self.position_to_cell(&bounds.max);

        let mut cells = Vec::new();

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                cells.push(Cell { x, y });
            }
        }

        cells
    }

    pub fn query(&self, bounds: &Bounds, world: &World) -> Vec<EntityId> {
        let mut result = Vec::new();

        for cell in self.cells_in_bounds(bounds) {
            let Some(entities) = self.cells.get(&cell) else {
                continue;
            };

            for id in entities {
                if let Some(position) = world.position(*id) {
                    if bounds.contains(position) {
                        result.push(*id);
                    }
                }
            }
        }

        result
    }

    pub fn move_entity(&mut self, id: EntityId, old_point: &Point3, new_point: &Point3) {
        let old_cell = self.position_to_cell(old_point);
        let new_cell = self.position_to_cell(new_point);

        if old_cell == new_cell {
            return;
        }

        if let Some(entities) = self.cells.get_mut(&old_cell) {
            entities.remove(&id);
            if entities.is_empty() {
                self.cells.remove(&old_cell);
            }
        }

        self.cells.entry(new_cell).or_default().insert(id);
    }
}
