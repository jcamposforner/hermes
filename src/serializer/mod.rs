use serde::Serialize;

use crate::event::{Event};
use crate::serializer::error::SerializeError;

pub mod serde_serialize;

mod error;
mod serialized_event;

pub trait EventSerializer: Send + Sync + 'static {
    fn serialize<T: Event + Serialize>(&self, event: &T) -> Result<String, SerializeError>;
}