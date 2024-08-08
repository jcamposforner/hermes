use lapin::{Connection, ConnectionProperties};

use hermes::rabbit::rabbit_configurer::RabbitConfigurer;

#[tokio::main]
async fn main() {
    let configurer = RabbitConfigurer::new(
        Connection::connect("amqp://localhost:5672", ConnectionProperties::default()).await.unwrap(),
        "exchange".to_string(),
        1000
    );

    configurer.configure(
        (
            "update_user_total_messages_on_event",
            &["user_sent_message", "user_removed_message"]
        )).await;
}
