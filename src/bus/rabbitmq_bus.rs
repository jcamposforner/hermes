use std::error::Error;
use std::sync::Arc;

use lapin::{BasicProperties, Channel, Connection};
use serde::Serialize;
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::Event;
use crate::serializer::EventSerializer;

pub struct RabbitEventBus<'a, T: EventSerializer> {
    connection: Connection,
    serializer: &'a T,
    exchange: String,
    channel: Arc<RwLock<Option<Channel>>>,
}

impl<'a, T: EventSerializer> RabbitEventBus<'a, T> {
    pub async fn new(connection: Connection, serializer: &'a T, exchange: String) -> Result<Self, Box<dyn Error>> {
        let event_bus = Self {
            connection,
            serializer,
            exchange,
            channel: Arc::new(RwLock::new(None)),
        };

        event_bus.recreate_channel().await?;

        Ok(event_bus)
    }
}

impl<T: EventSerializer> AsynchronousEventBus for RabbitEventBus<'_, T> {
    async fn publish<E: Event + Serialize>(&self, event: E) -> Result<(), PublishError> {
        let read_guard = self.get_guard_channel().await?;

        let channel = read_guard
            .as_ref()
            .ok_or(PublishError::CannotOpenChannel)?;

        self.publish_message(&event, channel).await
    }
}

impl<T: EventSerializer> RabbitEventBus<'_, T> {
    async fn publish_message<E: Event + Serialize>(&self, event: &E, channel: &Channel) -> Result<(), PublishError> {
        let payload = self.serializer.serialize(event).map_err(|_| PublishError::CannotSerializeEvent)?;

        let publish_message = channel
            .basic_publish(
                self.exchange.as_str(),
                event.event_name(),
                lapin::options::BasicPublishOptions::default(),
                payload.into_bytes(),
                BasicProperties::default(),
            );

        publish_message
            .await
            .map_err(|_| PublishError::CannotOpenChannel)?
            .await
            .map_err(|_| PublishError::CannotPublishEvent)?;
        Ok(())
    }
}

impl<T: EventSerializer> RabbitEventBus<'_, T> {
    async fn recreate_channel(&self) -> Result<(), PublishError> {
        let mut write_guard = self.channel.write().await;
        if write_guard.is_none() {
            let channel = self.connection.create_channel().await.map_err(|_| PublishError::CannotOpenChannel)?;
            *write_guard = Some(channel);
        }

        drop(write_guard);

        Ok(())
    }

    async fn get_guard_channel(&self) -> Result<RwLockReadGuard<Option<Channel>>, PublishError> {
        let read_guard = self.channel.read().await;

        let channel = read_guard
            .as_ref()
            .ok_or(PublishError::CannotOpenChannel)?;

        let is_connected = channel.status().connected();
        if !is_connected {
            drop(read_guard);
            self.recreate_channel().await?;
        }

        let read_guard = self.channel.read().await;
        Ok(read_guard)
    }
}

#[cfg(test)]
mod tests {
    use lapin::ConnectionProperties;
    use tokio::time::Instant;

    use crate::async_publish_all;
    use crate::serializer::serde_serialize::SerdeJSONEventSerializer;

    use super::*;

    #[derive(Serialize)]
    struct TestEvent {}

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            "test_event"
        }
    }

    #[derive(Serialize)]
    struct OtherTestEvent {}

    impl Event for OtherTestEvent {
        fn event_name(&self) -> &'static str {
            "other_test_event"
        }
    }

    #[derive(Serialize)]
    enum TestEvents {
        TestEvent(TestEvent),
        OtherTestEvent(OtherTestEvent),
    }

    impl Event for TestEvents {
        fn event_name(&self) -> &'static str {
            match self {
                TestEvents::TestEvent(event) => event.event_name(),
                TestEvents::OtherTestEvent(event) => event.event_name(),
            }
        }
    }

    #[tokio::test]
    async fn test_publish() {
        let connection = Connection::connect("amqp://localhost", ConnectionProperties::default()).await.unwrap();

        let event_bus = Arc::new(
            RabbitEventBus::new(
                connection,
                &SerdeJSONEventSerializer,
                "new_exchange".to_string()
            ).await.unwrap()
        );

        let mut events = vec![];

        for _ in 0..10 {
            let event = TestEvents::TestEvent(TestEvent {});

            events.push(event);
            let event = TestEvents::OtherTestEvent(OtherTestEvent {});
            events.push(event);
        }


        async_publish_all!(event_bus, events);
    }
}