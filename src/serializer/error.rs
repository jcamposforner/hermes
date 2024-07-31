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
