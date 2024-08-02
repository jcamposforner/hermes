use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use hermes::consumer::PayloadHandler;
use hermes::derive::Event;
use hermes::event::Event;
use hermes::impl_payload_handler;
use hermes::serializer::deserialized_event::{EventDeserializable, EventDeserializableData, EventDeserializableMeta};

#[derive(Debug, Serialize, Deserialize, Event)]
struct ChatMessageSent {
    pub message: String,
    pub user: String,
}

#[derive(Debug, Serialize, Deserialize, Event)]
struct ChatMessageReceived {
    pub message: String,
}

struct SendNotificationOnChatMessageSent;

impl SendNotificationOnChatMessageSent {
    async fn on<T: Event + Debug>(&self, event: &T) {
        println!("Handling message event: {:?}", event);
    }
}

impl_payload_handler!(
    SendNotificationOnChatMessageSent,
    ChatMessageSent,
    ChatMessageReceived
);

#[tokio::main]
async fn main() {
    let event = ChatMessageSent {
        message: "new message".to_string(),
        user: "user".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    let mut handler = SendNotificationOnChatMessageSent;

    let message_from_rabbit = EventDeserializable {
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

    let message_from_rabbit = EventDeserializable {
        data: EventDeserializableData {
            event_name: event.event_name().to_string(),
            attributes: json,
        },
        meta: EventDeserializableMeta {},
    };

    handler.handle_value_payload(&message_from_rabbit).await.unwrap();
}
