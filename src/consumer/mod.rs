use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serializer::deserialized_event::EventDeserializable;

pub mod rabbitmq_consumer;
mod rabbitmq_retryer;

#[allow(async_fn_in_trait)]
pub trait AsyncConsumer {
    async fn consume(&mut self);
}

pub enum PayloadHandlerError {
    UnrecoverableError,
    Inner(Box<dyn Error>),
}

pub trait PayloadHandler<T: Serialize + DeserializeOwned> {
    fn handle(&mut self, payload: &EventDeserializable<T>) -> Result<(), PayloadHandlerError>;
}

macro_rules! impl_payload_handler {
    () => {};
}