#[cfg(feature = "rabbit")]
pub mod rabbitmq_consumer;

#[cfg(feature = "rabbit")]
pub trait AsyncConsumer {
    async fn consume(&mut self);
}