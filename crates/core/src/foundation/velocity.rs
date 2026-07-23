use crate::geometry::vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub linear: Vector3,
}

impl Velocity {
    pub fn new(linear: Vector3) -> Self {
        Self { linear }
    }
}
