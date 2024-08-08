use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use lapin::{Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};

use hermes::bus::AsynchronousEventBus;
use hermes::bus::rabbitmq_bus::RabbitEventBus;
use hermes::consumer::AsyncConsumer;
use hermes::consumer::rabbitmq_consumer::RabbitMQConsumer;
use hermes::consumer::rabbitmq_retryer::RabbitMQRetryer;
use hermes::derive::{Event, EventMetadata};
use hermes::event::EventMetadata;
use hermes::impl_payload_handler;
use hermes::rabbit::rabbit_publisher::RabbitPublisher;
use hermes::serializer::serde_formatter::SerdeJSONEventFormatter;
use hermes::subscriber::SubscriberError;

#[derive(Debug, EventMetadata, Serialize, Deserialize, Event)]
struct ChatMessageSent {
    pub message: String,
    pub user: String,
    pub metadata: EventMetadata
}

#[derive(Debug, Serialize, Deserialize, Event)]
struct ChatMessageReceived {
    pub message: String,
    pub metadata: EventMetadata
}

#[derive(Debug)]
struct SendNotificationOnChatMessageSentError;

impl Display for SendNotificationOnChatMessageSentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendNotificationOnChatMessageSentError")
    }
}

impl Error for SendNotificationOnChatMessageSentError {}

struct SendNotificationOnChatMessageSent;

impl SendNotificationOnChatMessageSent {
    async fn on_chat_message_sent(&self, event: &ChatMessageSent) -> Result<(), SubscriberError> {
        println!("Handling message sent: {:?}", event);

        Err(SubscriberError::Inner(SendNotificationOnChatMessageSentError.into()))
    }

    async fn on_chat_message_received(&self, event: &ChatMessageReceived) -> Result<(), SubscriberError> {
        println!("Handling message received: {:?}", event);
        Ok(())
    }
}

impl_payload_handler!(
    SendNotificationOnChatMessageSent,
    (ChatMessageSent, on_chat_message_sent),
    (ChatMessageReceived, on_chat_message_received)
);

#[tokio::main]
async fn main() {
    let connection = Arc::new(Connection::connect("amqp://localhost", ConnectionProperties::default()).await.unwrap());
    let formatter = SerdeJSONEventFormatter;
    let publisher = Arc::new(RabbitPublisher::new(connection.clone()).await.unwrap());

    let event_bus = RabbitEventBus::new(publisher.clone(), &formatter, "chat".to_string()).await;

    let mut metadata = EventMetadata::default();
    metadata.add("correlation-id".to_string(), "498404fa-0946-4be0-84f7-0a994c61fd77".to_string());

    let event = ChatMessageSent {
        message: "new message".to_string(),
        user: "user".to_string(),
        metadata
    };

    event_bus.publish(event).await.expect("TODO: panic message");

    let retryer = RabbitMQRetryer::new(publisher, 3);
    let mut consumer = RabbitMQConsumer::new(
        connection,
        "SendNotificationOnChatMessageSent",
        "SendNotificationOnChatMessageSentTag",
        &formatter,
        SendNotificationOnChatMessageSent,
        &retryer
    ).await.unwrap();

    consumer.consume().await;
}
