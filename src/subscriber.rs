use crate::event::Event;

pub trait Subscriber<T: Event> {
    fn handle_event(&mut self, event: &T);
}