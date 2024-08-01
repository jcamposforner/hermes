#![warn(clippy::all)]

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serializer::deserialized_event::EventDeserializable;

pub mod event;
pub mod subscriber;
pub mod bus;

#[cfg(feature = "serializer")]
pub mod serializer;
pub mod consumer;

#[cfg(feature = "rabbit")]
pub mod rabbit;

#[cfg(feature = "derive")]
pub mod derive {
    pub use hermes_derive::Event;
}

pub trait PayloadHandler<T: Serialize + DeserializeOwned> {
    fn handle(&mut self, payload: &EventDeserializable<T>);
}
