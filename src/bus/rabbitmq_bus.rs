use std::error::Error;
use std::sync::{Arc};

use lapin::{BasicProperties, Channel, Connection};
use serde::Serialize;
use tokio::sync::RwLock;
use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::{Event, EventIdentifiable};
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

    async fn recreate_channel(&self) -> Result<(), PublishError> {
        let mut write_guard = self.channel.write().await;
        if write_guard.is_none() {
            let channel = self.connection.create_channel().await.map_err(|_| PublishError::CannotOpenChannel)?;
            *write_guard = Some(channel);
        }

        drop(write_guard);

        Ok(())
    }
}

impl<T: EventSerializer> AsynchronousEventBus for RabbitEventBus<'_, T> {
    async fn publish<E: Event + EventIdentifiable + Serialize>(&self, event: E) -> Result<(), PublishError> {
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

        let channel = read_guard
            .as_ref()
            .ok_or(PublishError::CannotOpenChannel)?;

        let payload = self.serializer.serialize(&event).map_err(|_| PublishError::CannotSerializeEvent)?;
        let publish_message = channel
            .basic_publish(
                self.exchange.as_str(),
                E::event_name(),
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

#[cfg(test)]
mod tests {
    use lapin::ConnectionProperties;

    use crate::serializer::serde_serialize::SerdeJSONEventSerializer;

    use super::*;

    #[derive(Serialize)]
    struct TestEvent {}

    impl Event for TestEvent {}

    impl EventIdentifiable for TestEvent {
        fn event_name() -> &'static str {
            "test_event"
        }
    }

    #[tokio::test]
    async fn test_publish() {
        let connection = Connection::connect("amqp://localhost", ConnectionProperties::default()).await.unwrap();

        let event_bus = RabbitEventBus::new(connection, &SerdeJSONEventSerializer, "new_exchange".to_string()).await.unwrap();

        for _ in 0..1000 {
            let event = TestEvent {};

            event_bus.publish(event).await.unwrap();
        }
    }
}