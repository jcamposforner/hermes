use std::error::Error;
use std::fmt::Display;

use crate::event::Event;

pub trait Subscriber<T: Event> {
    fn handle_event(&self, event: &T) -> Result<(), SubscriberError>;
}

#[cfg(feature = "async")]
pub trait AsyncSubscriber<T: Event>: Send + Sync + 'static {
    fn handle_event(&self, event: &T) -> impl std::future::Future<Output = Result<(), SubscriberError>> + Send;
}

#[derive(Debug)]
pub enum SubscriberError {
    UnrecoverableError,
    Inner(Box<dyn Error>),
}

impl Display for SubscriberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriberError::UnrecoverableError => write!(f, "Unrecoverable error"),
            SubscriberError::Inner(e) => write!(f, "Inner error: {}", e),
        }
    }
}

impl Error for SubscriberError {}