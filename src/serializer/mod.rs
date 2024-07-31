use serde::{Serialize};
use serde_json::Value;
use crate::event::{Event, EventIdentifiable};
use crate::serializer::error::{DeserializeError, SerializeError};

pub mod serde_serialize;
pub mod serde_deserialize;
mod error;
mod event_registry;

pub trait EventSerializer {
    fn serialize<T: Event + EventIdentifiable + Serialize>(&self, event: &T) -> Result<String, SerializeError>;
}

pub trait EventDeserializer {
    fn deserialize(&self, raw_event: &str) -> Result<Box<dyn Event>, DeserializeError>;
}

pub trait EventDeserialized {
    fn from_value(value: Value) -> Result<Box<dyn Event>, DeserializeError>;
}

#[derive(Serialize)]
pub struct EventSerializable<'a, T: Event + Serialize> {
    event_name: &'a str,
    payload: &'a T,
}

impl<'a, T: Event + Serialize> EventSerializable<'a, T> {
    pub fn new(event_name: &'a str, payload: &'a T) -> Self {
        Self {
            event_name,
            payload
        }
    }
}