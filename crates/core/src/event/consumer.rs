use crate::event::event::Event;

pub trait EventConsumer {
    fn consume(&mut self, event: &Event);
}
