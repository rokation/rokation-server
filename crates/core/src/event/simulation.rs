use serde::Serialize;

use crate::sensor::detection::Detection;

#[derive(Debug, Serialize)]
pub enum SimulationEvent {
    Detection(Detection),
}
