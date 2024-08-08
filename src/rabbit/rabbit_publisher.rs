use std::error::Error;
use std::sync::Arc;

use lapin::{BasicProperties, Channel, Connection};
use lapin::types::FieldTable;
use tokio::sync::RwLockReadGuard;

use crate::bus::error::PublishError;
use crate::rabbit::rabbit_channel::RabbitChannel;

pub struct RabbitPublisher {
    channel: RabbitChannel,
}

impl RabbitPublisher {
    pub async fn new(connection: Arc<Connection>) -> Result<Self, Box<dyn Error>> {
        let channel = connection.create_channel().await?;

        Ok(
            RabbitPublisher {
                channel: RabbitChannel::new(connection, channel),
            }
        )
    }

    pub async fn publish(&self, payload: &[u8], routing_key: &str, exchange: &str) -> Result<(), PublishError> {
        let channel = self.get_guard_channel().await?;

        let publish_message = channel
            .basic_publish(
                exchange,
                routing_key,
                lapin::options::BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            );

        publish_message
            .await
            .map_err(|_| PublishError::CannotOpenChannel)?
            .await
            .map_err(|_| PublishError::CannotPublishEvent)?;

        Ok(())
    }

    pub async fn publish_with_headers(&self, payload: &[u8], routing_key: &str, exchange: &str, headers: FieldTable) -> Result<(), PublishError> {
        let channel = self.get_guard_channel().await?;

        let publish_message = channel
            .basic_publish(
                exchange,
                routing_key,
                lapin::options::BasicPublishOptions::default(),
                payload,
                BasicProperties::default()
                    .with_headers(headers),
            );

        publish_message
            .await
            .map_err(|_| PublishError::CannotOpenChannel)?
            .await
            .map_err(|_| PublishError::CannotPublishEvent)?;

        Ok(())
    }

    async fn get_guard_channel(&self) -> Result<RwLockReadGuard<Channel>, PublishError> {
        self.channel.get_guard_channel()
            .await
            .map_err(|_| PublishError::CannotOpenChannel)
    }
}
