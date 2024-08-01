use std::error::Error;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serializer::deserialized_event::EventDeserializable;

#[cfg(feature = "rabbit")]
pub mod rabbitmq_consumer;

#[cfg(feature = "rabbit")]
#[allow(async_fn_in_trait)]
pub trait AsyncConsumer {
    async fn consume(&mut self);
}

pub enum PayloadHandlerError {
    UnrecoverableError,
    Inner(Box<dyn Error>),
}

#[cfg(feature = "rabbit")]
pub trait PayloadHandler<T: Serialize + DeserializeOwned> {
    fn handle(&mut self, payload: &EventDeserializable<T>) -> Result<(), PayloadHandlerError>;
}
