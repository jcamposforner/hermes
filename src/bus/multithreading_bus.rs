use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use downcaster::{Downcast, downcast_ref};
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::bus::EventBus;
use crate::event::Event;
use crate::subscriber::Subscriber;

#[derive(Debug)]
pub enum MultithreadingEventBusError {
    ThreadPoolError(String),
}

impl std::fmt::Display for MultithreadingEventBusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MultithreadingEventBusError::ThreadPoolError(error) => write!(f, "ThreadPoolError: {}", error),
        }
    }
}

impl std::error::Error for MultithreadingEventBusError {}

///
/// A closure that takes an event and returns nothing.
///
type SubscriberClosure = Arc<dyn Fn(&dyn Event) + Send + Sync>;

///
/// A multithreading event bus that uses a thread pool to handle events.
///
pub struct MultithreadingEventBus {
    subscribers: HashMap<TypeId, Vec<SubscriberClosure>>,
    thread_pool: ThreadPool,
}

impl Default for MultithreadingEventBus {
    fn default() -> Self {
        Self {
            subscribers: HashMap::default(),
            thread_pool: ThreadPoolBuilder::new().build().expect("Error creating thread pool"),
        }
    }
}

impl MultithreadingEventBus {
    ///
    /// Create a new MultithreadingEventBus with a given thread pool.
    ///
    pub fn new(thread_pool: ThreadPool) -> Self {
        Self {
            thread_pool,
            ..Default::default()
        }
    }

    ///
    /// Create a new MultithreadingEventBus with a given number of threads.
    ///
    pub fn with_num_threads(threads: usize) -> Result<Self, MultithreadingEventBusError> {
        let thread_pool_result = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build();

        let thread_pool = match thread_pool_result {
            Ok(thread_pool) => thread_pool,
            Err(error) => return Err(MultithreadingEventBusError::ThreadPoolError(format!("Error creating thread pool: {}", error))),
        };

        Ok(Self::new(thread_pool))
    }

    ///
    /// Register a subscriber for a given event type.
    ///
    pub fn register<E: Event + Downcast + 'static, T: Subscriber<E> + Send + Sync + 'static>(&mut self, subscriber: Arc<T>) {
        let event_type = TypeId::of::<E>();

        let handler: SubscriberClosure = Arc::new(move |event| {
            downcast_ref!(event, E)
                .map(|event| {
                    match subscriber.handle_event(event) {
                        Ok(_) => {},
                        Err(e) => {
                            log::error!("Error while processing event: {:?}", e);
                        }
                    }
                });
        });

        self.subscribers
            .entry(event_type)
            .or_default()
            .push(handler);
    }
}

impl EventBus for MultithreadingEventBus {
    ///
    /// Publish an event to all subscribers.
    ///
    fn publish<E: Event>(&self, event: E) {
        let event_type = event.type_id();
        let event = Arc::new(event);

        if let Some(handlers) = self.subscribers.get(&event_type) {
            for handler in handlers {
                let handler = handler.clone();
                let event = event.clone();
                self.thread_pool.spawn(move || {
                    handler(event.as_ref());
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, Sender};
    use std::thread::sleep;
    use std::time::Duration;

    use tokio::time::Instant;

    use crate::impl_event_handler;
    use crate::subscriber::SubscriberError;

    use super::*;

    struct TestEvent {
        tx: Sender<bool>
    }

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            "test_event"
        }
    }

    struct TestEventHandler {}

    impl TestEventHandler {
        fn on(&self, _event: &TestEvent) -> Result<(), SubscriberError> {
            sleep(Duration::from_secs(1));
            let _ = _event.tx.send(true);

            Ok(())
        }
    }

    impl_event_handler!(TestEventHandler, TestEvent);

    #[test]
    fn it_should_create_a_thread_system_and_notify_and_dont_block() {
        let mut event_bus = MultithreadingEventBus::with_num_threads(4).unwrap();

        event_bus.register(Arc::new(TestEventHandler {}));

        let (tx, rx) = channel();
        let now = Instant::now();
        event_bus.publish(TestEvent { tx: tx.clone() });
        event_bus.publish(TestEvent { tx: tx.clone() });

        drop(tx);
        while rx.recv().is_ok() {}
        let elapsed = now.elapsed();

        assert!(elapsed.as_secs() < 2);
    }
}