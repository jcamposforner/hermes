use hermes::derive::{Event, Metadata};
use hermes::event::{Event, EventMetadata, EventWithMetadata};

#[derive(Event, Metadata, Debug)]
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

fn main() {
    let mut message = ChatMessageSent::new("new message".to_string(), "user".to_string());

    message.add_metadata("key".to_string(), "value".to_string());

    println!("Event Name: {:?}", message.event_name());
    println!("Event Name: {:?}", message);
}
