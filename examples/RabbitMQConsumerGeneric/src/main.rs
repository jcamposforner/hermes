use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use hermes::consumer::PayloadHandler;
use hermes::consumer::PayloadHandlerError;
use hermes::event::Event;
use hermes::impl_async_event_handler;
use hermes::impl_payload_handler;
use hermes::serializer::deserialized_event::{EventDeserializable, EventDeserializableData, EventDeserializableMeta};
use hermes::subscriber::AsyncSubscriber;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageSent {
    pub message: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageReceived {
    pub message: String,
}

impl Event for ChatMessageSent {
    fn event_name(&self) -> &'static str {
        "chat_message_sent"
    }
}

impl Event for ChatMessageReceived {
    fn event_name(&self) -> &'static str {
        "chat_message_received"
    }
}

struct SendNotificationOnChatMessageSent;

impl SendNotificationOnChatMessageSent {
    async fn on<T: Event + Debug>(&self, event: &T) {
        println!("Handling message event: {:?}", event);
    }
}

impl_payload_handler!(
    SendNotificationOnChatMessageSent,
    ("chat_message_sent", ChatMessageSent),
    ("chat_message_received", ChatMessageReceived)
);

#[tokio::main]
async fn main() {
    let event = ChatMessageSent {
        message: "new message".to_string(),
        user: "user".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    let mut handler = SendNotificationOnChatMessageSent;

    let message_from_rabbit: EventDeserializable<Value> = EventDeserializable {
        data: EventDeserializableData {
            event_name: event.event_name().to_string(),
            attributes: json,
        },
        meta: EventDeserializableMeta {},
    };

    handler.handle_value_payload(&message_from_rabbit).await.unwrap();

    let event = ChatMessageReceived {
        message: "new message".to_string()
    };
    let json = serde_json::to_value(&event).unwrap();

    let message_from_rabbit: EventDeserializable<Value> = EventDeserializable {
        data: EventDeserializableData {
            event_name: event.event_name().to_string(),
            attributes: json,
        },
        meta: EventDeserializableMeta {},
    };

    handler.handle_value_payload(&message_from_rabbit).await.unwrap();
}
