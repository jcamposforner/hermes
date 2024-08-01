use std::sync::Arc;
use lapin::{Channel, Connection};
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use futures_lite::stream::StreamExt;
use log::info;
use crate::consumer::AsyncConsumer;

pub struct RabbitMQConsumer {
    connection: Arc<Connection>,
    channel: Channel,
    queue: String,
    consumer_tag: String,
}

impl RabbitMQConsumer {
    pub async fn new<'a>(connection: Arc<Connection>, queue: &'a str, consumer_tag: &'a str) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = connection.create_channel().await?;

        Ok(
            Self {
                connection,
                channel,
                queue: queue.to_string(),
                consumer_tag: consumer_tag.to_string(),
            }
        )
    }
}

impl AsyncConsumer for RabbitMQConsumer {
    async fn consume(&mut self) {
        let mut consumer = self.channel
                               .basic_consume(
                                   &self.queue,
                                   &self.consumer_tag,
                                   BasicConsumeOptions::default(),
                                   FieldTable::default(),
                               )
                               .await
                               .unwrap();

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let payload = std::str::from_utf8(&delivery.data).unwrap();
                info!("Received message: {:?}", payload);

                self.channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consume() {
        let connection = Connection::connect("amqp://localhost", lapin::ConnectionProperties::default()).await.unwrap();

        let mut consumer = RabbitMQConsumer::new(Arc::new(connection), "q", "test_consumer").await.unwrap();

        consumer.consume().await;
    }
}