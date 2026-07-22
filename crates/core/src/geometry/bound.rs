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

    pub fn intersects(&self, other: &Bound) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bound() {
        let bound = Bound {
            min: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Point3 {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
        };

        let p1 = Point3 {
            x: 50.0,
            y: 50.0,
            z: 50.0,
        };

        let p2 = Point3 {
            x: 150.0,
            y: 150.0,
            z: 150.0,
        };

        assert!(bound.contains(&p1));
        assert!(!bound.contains(&p2));
    }

    #[test]
    fn test_bound_intersects() {
        let b1 = Bound {
            min: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Point3 {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
        };

        let b2 = Bound {
            min: Point3 {
                x: 50.0,
                y: 50.0,
                z: 50.0,
            },
            max: Point3 {
                x: 150.0,
                y: 150.0,
                z: 150.0,
            },
        };

        let b3 = Bound {
            min: Point3 {
                x: 200.0,
                y: 200.0,
                z: 200.0,
            },
            max: Point3 {
                x: 300.0,
                y: 300.0,
                z: 300.0,
            },
        };

        assert!(b1.intersects(&b2));
        assert!(!b2.intersects(&b3));
    }
}
