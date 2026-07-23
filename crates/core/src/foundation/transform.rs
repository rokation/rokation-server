use crate::foundation::{position::Position, rotation::Rotation};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Position,
    pub rotation: Rotation,
}

impl Transform {
    pub fn new(position: Position, rotation: Rotation) -> Self {
        Self { position, rotation }
    }

    pub fn identity() -> Self {
        Self {
            position: Position::zero(),
            rotation: Rotation::zero(),
        }
    }
}
