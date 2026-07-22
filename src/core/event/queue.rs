use crate::core::event::event::Event;

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<Event> {
        self.events.drain(..).collect()
    }
}
