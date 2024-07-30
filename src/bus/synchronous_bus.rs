use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use downcaster::{Downcast, downcast_ref};
use crate::bus::EventBus;
use crate::event::Event;
use crate::subscriber::Subscriber;

type SubscriberClosure = Box<dyn FnMut(&dyn Event)>;

pub struct SynchronousEventBus {
    subscribers: HashMap<TypeId, Vec<SubscriberClosure>>
}

impl EventBus for SynchronousEventBus {
    fn register<E, S>(&mut self, subscriber: Rc<S>)
    where
        E: Event + Downcast + 'static,
        S: Subscriber<E> + 'static
    {
        let event_type = TypeId::of::<E>();

        let handler: SubscriberClosure = Box::new(move |event| {
            downcast_ref!(event, E)
                .map(|event| {
                    subscriber.handle_event(event);
                });
        });

        self.subscribers
            .entry(event_type)
            .or_insert(Vec::new())
            .push(handler);
    }

    fn publish<T: Event>(&mut self, event: &T) {
        let event_type = TypeId::of::<T>();

        if let Some(handlers) = self.subscribers.get_mut(&event_type) {
            for handler in handlers {
                handler(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

    struct TestEvent {}

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            return "test_event";
        }
    }

    struct OtherTestEvent {}

    impl Event for OtherTestEvent {
        fn event_name(&self) -> &'static str {
            return "other_test_event";
        }
    }

    struct TestEventHandler {
        total_messages_received: RefCell<u32>
    }

    impl Subscriber<TestEvent> for TestEventHandler {
        fn handle_event(&self, _event: &TestEvent) {
            *self.total_messages_received.borrow_mut() += 1;
        }
    }

    #[test]
    fn it_works() {
        let mut event_bus = SynchronousEventBus {
            subscribers: HashMap::new()
        };

        let handler = Rc::new(
            TestEventHandler { total_messages_received: RefCell::new(0) }
        );
        event_bus.register::<TestEvent, TestEventHandler>(handler.clone());

        event_bus.publish(&TestEvent {});
        event_bus.publish(&OtherTestEvent {});

        assert_eq!(*handler.clone().total_messages_received.borrow(), 1);
    }
}