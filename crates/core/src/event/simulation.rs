use serde::Serialize;

use crate::sensor::detection::Detection;

#[derive(Debug, Serialize)]
pub enum SimulationEvent {
    Detection(Detection),
}

#[derive(Debug, Serialize)]
pub struct Event {
    pub tick: u64,
    pub time: f64,
    pub kind: SimulationEvent,
}
