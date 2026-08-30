use serde::{Deserialize, Serialize};

use crate::geometry::vector::Point3;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: Point3,
}
