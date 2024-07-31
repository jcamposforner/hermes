use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SerializeError {
    UnableToSerializeEvent
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::UnableToSerializeEvent => write!(f, "Unable to serialize event"),
        }
    }
}

impl Error for SerializeError {}

#[derive(Debug)]
pub enum DeserializeError {
    UnableToDowncastEvent,
    UnableToDeserializeEvent,
    MissingEventName
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::UnableToDeserializeEvent => write!(f, "Unable to deserialize event"),
            DeserializeError::MissingEventName => write!(f, "Unable to find event name"),
            DeserializeError::UnableToDowncastEvent => write!(f, "Unable to downcast event name"),
        }
    }
}

impl Error for DeserializeError {}