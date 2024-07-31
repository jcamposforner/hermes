use crate::event::Event;

pub mod synchronous_bus;

#[cfg(feature = "async")]
pub mod asynchronous_bus;

#[cfg(feature = "multithreading")]
pub mod multithreading_bus;

pub trait EventBus {
    fn publish<E: Event>(&self, event: E);
}

#[cfg(feature = "async")]
#[allow(async_fn_in_trait)]
pub trait AsynchronousEventBus {
    async fn publish<E: Event>(&self, event: E);
}

#[macro_export]
macro_rules! impl_event_handler {
    ($struct_name:ident, $method_name:ident, $($event:ident), *) => {
        $(
            impl Subscriber<$event> for $struct_name {
                fn handle_event(&self, event: &$event) {
                    self.$method_name(event);
                }
            }
        )*
    };
    ($struct_name:ident, $($event:ident), *) => {
        $(
            impl Subscriber<$event> for $struct_name {
                fn handle_event(&self, event: &$event) {
                    self.on(event);
                }
            }
        )*
    };
    ($struct_name:ident) => {
        impl<T: Event> Subscriber<T> for $struct_name {
            fn handle_event(&self, event: &T) {
                self.on(event);
            }
        }
    };
}