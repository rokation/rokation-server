use crate::event::simulation::SimulationEvent;

pub struct EventBus {
    events: Vec<SimulationEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn publish(&mut self, event: SimulationEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<SimulationEvent> {
        std::mem::take(&mut self.events)
    }
}
