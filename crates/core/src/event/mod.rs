use serde::Serialize;

use crate::event::simulation::SimulationEvent;

pub mod bus;
pub mod simulation;

#[derive(Debug, Serialize)]
pub struct Event {
    pub tick: u64,
    pub time: f64,
    pub kind: SimulationEvent,
}
