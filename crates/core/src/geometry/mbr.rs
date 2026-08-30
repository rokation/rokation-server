use crate::geometry::vector::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Mbr {
    pub min: Point3,
    pub max: Point3,
}

impl Mbr {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn from_point(point: Point3) -> Self {
        Mbr {
            min: point,
            max: point,
        }
    }

    pub fn from_radius(center: Point3, radius: f64) -> Self {
        Self {
            min: Point3::new(center.x - radius, center.y - radius, center.z - radius),
            max: Point3::new(center.x + radius, center.y + radius, center.z + radius),
        }
    }

    pub fn intersects(&self, other: &Mbr) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn expand(&self, other: &Mbr) -> Self {
        Self {
            min: Point3 {
                x: self.min.x.min(other.min.x),
                y: self.min.y.min(other.min.y),
                z: self.min.z.min(other.min.z),
            },

            max: Point3 {
                x: self.max.x.min(other.max.x),
                y: self.max.y.min(other.max.y),
                z: self.max.z.min(other.max.z),
            },
        }
    }

    pub fn area(&self) -> f64 {
        let width = self.max.x - self.min.x;
        let height = self.max.y - self.min.y;
        let depth = self.max.z - self.min.z;

        width * height * depth
    }

    pub fn enlargement(&self, other: &Mbr) -> f64 {
        let expanded = self.expand(other);

        expanded.area() - self.area()
    }
}
