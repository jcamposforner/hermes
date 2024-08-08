use std::sync::Arc;

use futures_lite::stream::StreamExt;
use lapin::Connection;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use log::error;
use serde_json::Value;

use crate::consumer::{AsyncConsumer, PayloadHandler, SubscriberError};
use crate::consumer::rabbitmq_retryer::RabbitMQRetryer;
use crate::rabbit::rabbit_channel::RabbitChannel;
use crate::serializer::EventDeserializer;

pub struct RabbitMQConsumer<'a, D: EventDeserializer, EH: PayloadHandler<Value>> {
    channel: RabbitChannel,
    queue: String,
    consumer_tag: String,
    deserializer: &'a D,
    handler: EH,
    retryer: &'a RabbitMQRetryer,
}

impl<'a, D: EventDeserializer, EH: PayloadHandler<Value>> RabbitMQConsumer<'a, D, EH> {
    pub async fn new(
        connection: Arc<Connection>,
        queue: &'a str,
        consumer_tag: &'a str,
        deserializer: &'a D,
        handler: EH,
        retryer: &'a RabbitMQRetryer
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = connection.create_channel().await?;

        Ok(
            Self {
                channel: RabbitChannel::new(connection, channel),
                queue: queue.to_string(),
                consumer_tag: consumer_tag.to_string(),
                deserializer,
                handler,
                retryer
            }
        )
    }
}

impl<'a, D: EventDeserializer, EH: PayloadHandler<Value>> AsyncConsumer for RabbitMQConsumer<'a, D, EH> {
    async fn consume(&mut self) {
        let channel = self.channel.get_guard_channel().await.unwrap();
        let mut consumer = channel
            .basic_consume(
                &self.queue,
                &self.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        while let Some(delivery) = consumer.next().await {
            if let Ok(mut delivery) = delivery {
                let payload = std::str::from_utf8(&delivery.data).unwrap();

                let event_deserializable = self.deserializer
                                               .deserialize::<Value>(payload.to_string());

                if event_deserializable.is_err() {
                    error!("Failed to deserialize event {}", payload);
                    continue;
                }

                let result = self.handler.handle_value_payload(&event_deserializable.expect("Failed to deserialize event")).await;

                match result {
                    Ok(_) | Err(SubscriberError::UnrecoverableError) => {
                        channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                               .await
                            .expect("Failed to acknowledge message");
                    },
                    Err(SubscriberError::Inner(_)) => {
                        self.retryer
                            .retry(&delivery, self.queue.as_str())
                            .await
                            .expect("Failed to retry message");

                        channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                               .await
                            .expect("Failed to acknowledge message");
                    }
                }
            }
        }
    }
}
