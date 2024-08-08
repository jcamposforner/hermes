use std::any::TypeId;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use downcaster::{Downcast, downcast_ref};

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::Event;
use crate::subscriber::AsyncSubscriber;

type SubscriberClosure = Box<dyn Fn(Arc<dyn Event>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

///
/// An asynchronous event bus that handles events asynchronously.
///
#[derive(Default)]
pub struct TokioEventBus {
    subscribers: HashMap<TypeId, Vec<SubscriberClosure>>
}

impl TokioEventBus {
    pub fn register<E, S>(&mut self, subscriber: Arc<S>)
    where
        E: Event + Downcast + 'static,
        S: AsyncSubscriber<E> + 'static
    {
        let event_type = TypeId::of::<E>();

        let handler: SubscriberClosure = Box::new(move |event| {
            let value = subscriber.clone();

            Box::pin(async move {
                let event = event.as_ref();
                if let Some(event) = downcast_ref!(event, E) {
                    match value.handle_event(event).await {
                        Ok(_) => {},
                        Err(e) => {
                            log::error!("Error while processing event: {:?}", e);
                        }
                    }
                }
            })
        });

        self.subscribers
            .entry(event_type)
            .or_default()
            .push(handler);
    }
}

impl AsynchronousEventBus for TokioEventBus {
    async fn publish<T: Event>(&self, event: T) -> Result<(), PublishError> {
        let event_type = TypeId::of::<T>();
        let event = Arc::new(event);

        let mut join_handlers = vec![];
        if let Some(handlers) = self.subscribers.get(&event_type) {
            for handler in handlers {
                let event = event.clone();
                join_handlers.push(tokio::spawn(handler(event)));
            }
        }

        for join_handler in join_handlers {
            match join_handler.await {
                Ok(_) => {},
                Err(e) => {
                    log::error!("Error while processing event: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! impl_async_event_handler {
    ($struct_name:ident, $method_name:ident, $($event:ident), *) => {
        $(
            impl $crate::subscriber::AsyncSubscriber<$event> for $struct_name {
                async fn handle_event(&self, event: &$event) -> Result<(), $crate::subscriber::SubscriberError> {
                    self.$method_name(event).await
                }
            }
        )*
    };
    ($struct_name:ident, $($event:ident), *) => {
        $(
            impl $crate::subscriber::AsyncSubscriber<$event> for $struct_name {
                async fn handle_event(&self, event: &$event) -> Result<(), $crate::subscriber::SubscriberError> {
                    self.on(event).await
                }
            }
        )*
    };
    ($struct_name:ident) => {
        impl<T: $crate::Event> $crate::subscriber::AsyncSubscriber<T> for $struct_name {
            async fn handle_event(&self, event: &T) -> Result<(), $crate::subscriber::SubscriberError> {
                self.on(event).await
            }
        }
    };
}

#[macro_export]
macro_rules! async_publish_all {
    ($bus:expr, $events:expr) => {{
        let mut handles = vec![];

        for event in $events {
            let event_bus_clone = $bus.clone();
            let handle = tokio::spawn(async move {
                let _ = event_bus_clone.publish(event).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            match handle.await {
                Ok(_) => {},
                Err(e) => {
                    log::error!("Error while processing event: {:?}", e);
                }
            }
        }
    }};
    ($bus:expr, $($event:expr),+) => {{
        let mut handles = vec![];

        $(
            let event_bus_clone = $bus.clone();
            let handle = tokio::spawn(async move {
                let _ = event_bus_clone.publish($event).await;
            });
            handles.push(handle);
        )+

        for handle in handles {
            match handle.await {
                Ok(_) => {},
                Err(e) => {
                    log::error!("Error while processing event: {:?}", e);
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::mem;
    use std::time::Duration;

    use serde::Serialize;
    use tokio::sync::mpsc::Sender;
    use tokio::time::{Instant, sleep};

    use crate::event::{EventMetadata, EventWithMetadata};
    use crate::subscriber::SubscriberError;

    use super::*;

    #[derive(Serialize)]
    struct TestEvent {
        metadata: EventMetadata
    }

    impl EventWithMetadata for TestEvent {
        fn add_metadata(&mut self, key: String, value: String) {
            self.metadata.add(key, value);
        }

        fn get_metadata(&self, key: &str) -> Option<&String> {
            self.metadata.get(key)
        }

        fn metadata(&self) -> &EventMetadata {
            &self.metadata
        }

        fn drain_metadata(&mut self) -> EventMetadata {
            mem::take(&mut self.metadata)
        }
    }

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            "test_event"
        }
    }

    #[derive(Serialize)]
    struct OtherTestEvent {
        metadata: EventMetadata
    }

    impl EventWithMetadata for OtherTestEvent {
        fn add_metadata(&mut self, key: String, value: String) {
            self.metadata.add(key, value);
        }

        fn get_metadata(&self, key: &str) -> Option<&String> {
            self.metadata.get(key)
        }

        fn metadata(&self) -> &EventMetadata {
            &self.metadata
        }

        fn drain_metadata(&mut self) -> EventMetadata {
            mem::take(&mut self.metadata)
        }
    }


    impl Event for OtherTestEvent {
        fn event_name(&self) -> &'static str {
            "other_test_event"
        }
    }

    struct TestEventHandler {
        sender: Sender<u8>
    }

    impl TestEventHandler {
        async fn on_test_event(&self, _event: &TestEvent) -> Result<(), SubscriberError> {
            self.sender.send(1).await.unwrap();
            Ok(())
        }
    }

    struct OtherEventHandler {}

    impl OtherEventHandler {
        async fn on_test_event(&self, _event: &TestEvent) -> Result<(), SubscriberError> {
            Ok(())
        }
    }

    impl_async_event_handler!(OtherEventHandler, on_test_event, TestEvent);
    impl_async_event_handler!(TestEventHandler, on_test_event, TestEvent);

    #[tokio::test]
    async fn it_should_publish_with_tokio_green_threads() {
        let mut event_bus = TokioEventBus::default();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        event_bus.register(Arc::new(TestEventHandler {
            sender: tx
        }));

        event_bus.register(Arc::new(OtherEventHandler {}));
        let _ = event_bus.publish(TestEvent { metadata: EventMetadata::default() }).await;

        let total_messages_received = rx.recv().await.unwrap();

        assert_eq!(total_messages_received, 1);
    }

    struct SleepyEventHandler {
        sleep_time: Duration,
    }

    impl SleepyEventHandler {
        async fn on_test_event(&self, _event: &TestEvent) -> Result<(), SubscriberError> {
            sleep(self.sleep_time).await;
            Ok(())
        }
    }

    impl_async_event_handler!(SleepyEventHandler, on_test_event, TestEvent);

    #[tokio::test]
    async fn it_should_concurrently_do_all_events_with_sleep_of_one_second() {
        let mut event_bus = TokioEventBus::default();

        event_bus.register(Arc::new(SleepyEventHandler {
            sleep_time: Duration::from_secs(1)
        }));

        event_bus.register(Arc::new(OtherEventHandler {}));
        let event_bus = Arc::new(event_bus);
        let start = Instant::now();

        async_publish_all!(event_bus, TestEvent { metadata: EventMetadata::default()}, TestEvent { metadata: EventMetadata::default()}, OtherTestEvent { metadata: EventMetadata::default()});

        let duration = start.elapsed();

        assert!(duration.as_secs() < 2);
    }
}