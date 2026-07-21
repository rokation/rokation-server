#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_position() {
        let position = Position::new(10.0, 10.0, 0.0);

        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 10.0);
        assert_eq!(position.z, 0.0);
    }

    #[test]
    fn test_compare_position() {
        let position_a = Position::new(10.0, 10.0, 0.0);
        let position_b = Position::new(10.0, 10.0, 0.0);

        assert_eq!(position_a, position_b);
    }
}
