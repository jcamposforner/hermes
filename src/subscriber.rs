use crate::event::Event;

pub trait Subscriber<T: Event> {
    fn handle_event(&self, event: &T);
}