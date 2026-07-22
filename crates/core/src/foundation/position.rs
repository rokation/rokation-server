use crate::geometry::point::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub point: Point3,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            point: Point3 { x, y, z },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_position() {
        let position = Position::new(10.0, 10.0, 0.0);

        assert_eq!(position.point.x, 10.0);
        assert_eq!(position.point.y, 10.0);
        assert_eq!(position.point.z, 0.0);
    }

    #[test]
    fn test_compare_position() {
        let position_a = Position::new(10.0, 10.0, 0.0);
        let position_b = Position::new(10.0, 10.0, 0.0);

        assert_eq!(position_a, position_b);
    }
}
