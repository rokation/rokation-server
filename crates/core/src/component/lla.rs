use serde::Serialize;

#[derive(Copy, Clone, Debug, Default, Serialize, PartialEq)]
pub struct Lla {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

impl Lla {
    pub fn new(lat: f64, lon: f64, alt: f64) -> Self {
        Self { lat, lon, alt }
    }
}
