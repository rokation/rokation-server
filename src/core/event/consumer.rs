use crate::core::event::event::Event;

pub trait EventConsumer {
    fn consume(&mut self, event: &Event);
}
