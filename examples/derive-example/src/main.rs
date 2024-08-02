use hermes::derive::Event;
use hermes::event::Event;

#[derive(Event)]
struct ChatMessageSent {
    pub message: String,
    pub user: String,
}

impl ChatMessageSent {
    fn new(message: String, user: String) -> Self {
        Self {
            message,
            user
        }
    }
}

fn main() {
    let message = ChatMessageSent::new("new message".to_string(), "user".to_string());

    println!("Event Name: {:?}", message.event_name());
}
