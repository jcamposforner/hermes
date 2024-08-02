use std::error::Error;
use std::sync::Arc;

use lapin::{BasicProperties, Channel, Connection};
use serde::Serialize;
use tokio::sync::RwLockReadGuard;

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::{Event, EventWithMetadata};
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
    async fn publish<E: Event + EventWithMetadata + Serialize>(&self, event: E) -> Result<(), PublishError> {
        let channel = self.get_guard_channel().await?;

        self.publish_message(&event, &channel).await
    }
}

impl<T: EventSerializer> RabbitEventBus<'_, T> {
    async fn publish_message<E: Event + EventWithMetadata + Serialize>(&self, event: &E, channel: &Channel) -> Result<(), PublishError> {
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
