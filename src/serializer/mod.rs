use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::event::{Event, EventWithMetadata};
use crate::serializer::deserialized_event::EventDeserializable;
use crate::serializer::error::{DeserializeError, SerializeError};

pub mod serde_formatter;
pub mod deserialized_event;

mod error;
mod serialized_event;

pub trait EventSerializer: Send + Sync + 'static {
    fn serialize<T: Event + EventWithMetadata + Serialize>(&self, event: &T) -> Result<String, SerializeError>;
}

pub trait EventDeserializer: Send + Sync + 'static {
    fn deserialize<T: DeserializeOwned + Serialize>(&self, raw_event: String) -> Result<EventDeserializable<T>, DeserializeError>;
}