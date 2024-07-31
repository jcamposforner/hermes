use serde::Serialize;

use crate::event::{Event, EventIdentifiable};
use crate::serializer::error::SerializeError;

pub mod serde_serialize;

mod error;
mod serialized_event;

pub trait EventSerializer {
    fn serialize<T: Event + EventIdentifiable + Serialize>(&self, event: &T) -> Result<String, SerializeError>;
}