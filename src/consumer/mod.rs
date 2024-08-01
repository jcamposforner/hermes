#[cfg(feature = "rabbit")]
pub mod rabbitmq_consumer;

#[cfg(feature = "rabbit")]
#[allow(async_fn_in_trait)]
pub trait AsyncConsumer {
    async fn consume(&mut self);
}