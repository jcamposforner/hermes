pub mod synchronous_bus;

use std::sync::Arc;
use downcaster::Downcast;
use crate::event::Event;
use crate::subscriber::Subscriber;

pub trait EventBus {
    fn register<E: Event + Downcast + 'static, S: Subscriber<E> + 'static>(&mut self, subscriber: Arc<S>);
    fn publish<E: Event>(&mut self, event: &E);
}