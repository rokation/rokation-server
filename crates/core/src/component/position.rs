#[derive(Debug)]
pub struct Position {
    lat: f64,
    lon: f64,
    alt: f64,
}

impl Position {
    pub fn new(lat: f64, lon: f64, alt: f64) -> Self {
        Self { lat, lon, alt }
    }
}
