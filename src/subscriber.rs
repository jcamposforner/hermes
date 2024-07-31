use crate::event::Event;

pub trait Subscriber<T: Event> {
    fn handle_event(&self, event: &T);
}

#[cfg(feature = "async")]
pub trait AsyncSubscriber<T: Event>: Send + Sync + 'static {
    fn handle_event(&self, event: &T) -> impl std::future::Future<Output = ()> + Send;
}