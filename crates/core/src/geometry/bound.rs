use crate::geometry::point::Point3;

pub struct Bound {
    pub min: Point3,
    pub max: Point3,
}

impl Bound {
    pub fn contains(&self, point: &Point3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }
}
