use std::error::Error;
use std::sync::Arc;

use lapin::{BasicProperties, Channel, Connection};
use serde::Serialize;
use tokio::sync::RwLockReadGuard;

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::Event;
use crate::rabbit::rabbit_channel::RabbitChannel;
use crate::serializer::EventSerializer;

pub struct RabbitEventBus<'a, T: EventSerializer> {
    serializer: &'a T,
    exchange: String,
    channel: RabbitChannel,
}

impl<'a, T: EventSerializer> RabbitEventBus<'a, T> {
    pub async fn new(connection: Arc<Connection>, serializer: &'a T, exchange: String) -> Result<Self, Box<dyn Error>> {
        let channel = connection.create_channel().await?;

        let event_bus = Self {
            serializer,
            exchange,
            channel: RabbitChannel::new(connection, channel),
        };

        Ok(event_bus)
    }
}

impl<T: EventSerializer> AsynchronousEventBus for RabbitEventBus<'_, T> {
    async fn publish<E: Event + Serialize>(&self, event: E) -> Result<(), PublishError> {
        let channel = self.get_guard_channel().await?;

        self.publish_message(&event, &channel).await
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
                payload.as_bytes(),
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
    async fn get_guard_channel(&self) -> Result<RwLockReadGuard<Channel>, PublishError> {
        self.channel.get_guard_channel()
            .await
            .map_err(|_| PublishError::CannotOpenChannel)
    }
}

#[cfg(test)]
mod tests {
    use lapin::ConnectionProperties;

    use crate::async_publish_all;
    use crate::serializer::serde_formatter::SerdeJSONEventFormatter;

    use super::*;

    #[derive(Serialize)]
    struct TestEvent {}

    impl Event for TestEvent {
        fn event_name(&self) -> &'static str {
            "test_event"
        }
    }

    #[derive(Serialize)]
    enum TestEvents {
        TestEvent(TestEvent),
    }

    impl Event for TestEvents {
        fn event_name(&self) -> &'static str {
            match self {
                TestEvents::TestEvent(event) => event.event_name(),
            }
        }
    }

    #[tokio::test]
    async fn test_publish() {
        let connection = Connection::connect("amqp://localhost", ConnectionProperties::default()).await.unwrap();

        let event_bus = Arc::new(
            RabbitEventBus::new(
                Arc::new(connection),
                &SerdeJSONEventFormatter,
                "new_exchange".to_string()
            ).await.unwrap()
        );

        let events = vec![TestEvents::TestEvent(TestEvent {})];

        async_publish_all!(event_bus, events);
    }
}