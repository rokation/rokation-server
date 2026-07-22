use std::ops::Add;

use crate::geometry::vector::Vector3;

pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
