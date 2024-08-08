use lapin::{Connection, ConnectionProperties};

use hermes::derive::Event;
use hermes::rabbit::rabbit_configurer::RabbitConfigurer;

#[derive(Event)]
struct ChatMessageSent;

#[tokio::main]
async fn main() {
    let configurer = RabbitConfigurer::new(
        Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await.unwrap(),
        "exchange".to_string(),
        1000
    );

    configurer.configure::<ChatMessageSent>("test").await;
}
