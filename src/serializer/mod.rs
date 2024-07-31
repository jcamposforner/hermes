use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::event::{Event, EventIdentifiable};
use crate::serializer::error::SerializeError;

pub mod serde_serialize;
pub mod serde_deserialize;
mod error;
mod event_registry;

pub trait EventSerializer {
    fn serialize<T: Event + EventIdentifiable + Serialize>(&self, event: &T) -> Result<String, SerializeError>;
}

pub trait EventDeserializer {
    fn deserialize<T: Event + DeserializeOwned>(&self, raw_event: &str) -> T;
}

#[derive(Serialize)]
pub struct EventSerializable<'a, T: Event + Serialize> {
    event_name: &'a str,
    payload: &'a  T,
}

impl<'a, T: Event + Serialize> EventSerializable<'a, T> {
    pub fn new(event_name: &'a str, payload: &'a T) -> Self {
        Self {
            event_name,
            payload
        }
    }
}