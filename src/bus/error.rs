use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PublishError {
    CannotSerializeEvent,
    CannotOpenChannel,
    CannotPublishEvent,
}

impl Display for PublishError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error publishing")
    }
}

impl Error for PublishError {}