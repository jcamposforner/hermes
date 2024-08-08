#[cfg(feature = "rabbit")]
#[cfg(feature = "async")]
use serde::Serialize;

use crate::bus::error::PublishError;
use crate::event::{Event, EventWithMetadata};

pub mod synchronous_bus;

#[cfg(feature = "async")]
pub mod asynchronous_bus;

#[cfg(feature = "multithreading")]
pub mod multithreading_bus;

#[cfg(feature = "rabbit")]
pub mod rabbitmq_bus;
pub(crate) mod error;

pub trait EventBus {
    fn publish<E: Event>(&self, event: E);
}

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait AsynchronousEventBus {
    async fn publish<E: Event + EventWithMetadata + Serialize>(&self, event: E) -> Result<(), PublishError>;
}

#[macro_export]
macro_rules! impl_event_handler {
    ($struct_name:ident, $method_name:ident, $($event:ident), *) => {
        $(
            impl $crate::subscriber::Subscriber<$event> for $struct_name {
                fn handle_event(&self, event: &$event) -> Result<(), $crate::subscriber::SubscriberError> {
                    self.$method_name(event)
                }
            }
        )*
    };
    ($struct_name:ident, $($event:ident), *) => {
        $(
            impl $crate::subscriber::Subscriber<$event> for $struct_name {
                fn handle_event(&self, event: &$event) -> Result<(), $crate::subscriber::SubscriberError> {
                    self.on(event)
                }
            }
        )*
    };
    ($struct_name:ident) => {
        impl<T: $crate::event::Event> $crate::subscriber::Subscriber<T> for $struct_name {
            fn handle_event(&self, event: &T) -> Result<(), $crate::subscriber::SubscriberError> {
                self.on(event)
            }
        }
    };
}