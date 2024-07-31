use std::error::Error;

use lapin::{BasicProperties, Channel, Connection};
use serde::Serialize;

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::{Event, EventIdentifiable};
use crate::serializer::EventSerializer;

pub struct RabbitEventBus<'a, T: EventSerializer> {
    connection: Connection,
    serializer: &'a T,
    exchange: String,
    channel: Option<Channel>,
}

impl<'a, T: EventSerializer> RabbitEventBus<'a, T> {
    pub async fn new(connection: Connection, serializer: &'a T, exchange: String) -> Self {
        Self {
            connection,
            serializer,
            exchange,
            channel: None,
        }
    }

    async fn recreate_channel(&mut self) -> Result<Option<&Channel>, Box<dyn Error>> {
        match self.channel {
            Some(_) => {},
            None => {
                let channel = self.connection.create_channel().await?;
                self.channel = Some(channel);
            }
        };

        Ok(self.channel.as_ref())
    }
}

impl<T: EventSerializer> AsynchronousEventBus for RabbitEventBus<'_, T> {
    async fn publish<E: Event + EventIdentifiable + Serialize>(&self, event: E) -> Result<(), PublishError> {
        let channel = self.channel
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