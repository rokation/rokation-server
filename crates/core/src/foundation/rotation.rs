#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    pub yaw: f64,
}

impl Rotation {
    pub fn new(yaw: f64) -> Self {
        Self { yaw }
    }

    pub fn zero() -> Self {
        Self { yaw: 0.0 }
    }
}
