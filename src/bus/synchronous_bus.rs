use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

use downcaster::{Downcast, downcast_ref};

use crate::bus::EventBus;
use crate::event::Event;
use crate::subscriber::Subscriber;

type SubscriberClosure = Box<dyn Fn(&dyn Event)>;

///
/// A synchronous event bus that handles events synchronously.
/// 
#[derive(Default)]
pub struct SynchronousEventBus {
    subscribers: HashMap<TypeId, Vec<SubscriberClosure>>
}

impl SynchronousEventBus {
    pub fn new() -> Self {
        SynchronousEventBus {
            subscribers: HashMap::new()
        }
    }

    pub fn register<E, S>(&mut self, subscriber: Rc<S>)
    where
        E: Event + Downcast + 'static,
        S: Subscriber<E> + 'static
    {
        let event_type = TypeId::of::<E>();

        let handler: SubscriberClosure = Box::new(move |event| {
            downcast_ref!(event, E)
                .map(|event| {
                    // TODO: Handle errors
                    let _ = subscriber.handle_event(event);
                });
        });

        self.subscribers
            .entry(event_type)
            .or_default()
            .push(handler);
    }
}

impl EventBus for SynchronousEventBus {
    fn publish<T: Event>(&self, event: T) {
        let event_type = TypeId::of::<T>();

        if let Some(handlers) = self.subscribers.get(&event_type) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::impl_event_handler;
    use crate::subscriber::SubscriberError;

    use super::*;

    struct TestEvent {}

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            "test_event"
        }
    }

    struct OtherTestEvent {}

    impl Event for OtherTestEvent {
        fn event_name(&self) -> &'static str {
            "other_test_event"
        }
    }

    struct TestEventHandler {
        total_messages_received: RefCell<u32>
    }

    impl TestEventHandler {
        fn on_test_event(&self, _event: &TestEvent) -> Result<(), SubscriberError> {
            *self.total_messages_received.borrow_mut() += 1;
            Ok(())
        }
    }

    impl_event_handler!(TestEventHandler, on_test_event, TestEvent);

    #[test]
    fn it_should_publish_and_modify_state() {
        let mut event_bus = SynchronousEventBus {
            subscribers: HashMap::new()
        };

        let handler = Rc::new(TestEventHandler { total_messages_received: RefCell::new(0) });
        event_bus.register(handler.clone());

        event_bus.publish(TestEvent {});
        event_bus.publish(OtherTestEvent {});

        assert_eq!(*handler.clone().total_messages_received.borrow(), 1);
    }
}