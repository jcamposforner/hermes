use std::rc::Rc;

use hermes::{impl_event_handler, impl_payload_handler};
use hermes::bus::EventBus;
use hermes::bus::synchronous_bus::SynchronousEventBus;
use hermes::derive::{Event, EventMetadata};
use hermes::event::{Event, EventMetadata, EventWithMetadata};
use hermes::subscriber::SubscriberError;

#[derive(Event, EventMetadata, Debug)]
struct ChatMessageSent {
    pub message: String,
    pub user: String,
    pub metadata: EventMetadata
}

impl ChatMessageSent {
    fn new(message: String, user: String) -> Self {
        Self {
            message,
            user,
            metadata: EventMetadata::default()
        }
    }
}

struct UpdateTotalMessagesSent;

impl UpdateTotalMessagesSent {
    fn on_chat_message_sent(&self, event: &ChatMessageSent) -> Result<(), SubscriberError> {
        println!("Handling message sent: {:?}", event.metadata.get("key"));

        Ok(())
    }
}

impl_event_handler!(UpdateTotalMessagesSent, on_chat_message_sent, ChatMessageSent);

fn main() {
    let mut message = ChatMessageSent::new("new message".to_string(), "user".to_string());

    message.add_metadata("key".to_string(), "value".to_string());

    println!("Event Name: {:?}", message.event_name());
    println!("Event Name: {:?}", message);

    let mut event_bus = SynchronousEventBus::new();
    event_bus.register(Rc::new(UpdateTotalMessagesSent));

    event_bus.publish(message);
}
