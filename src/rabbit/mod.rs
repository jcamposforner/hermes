use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod rabbit_channel;


#[derive(Debug)]
pub enum RabbitError {
    CannotOpenChannel,
}

impl Display for RabbitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RabbitError::CannotOpenChannel => write!(f, "Cannot open channel"),
        }
    }
}

impl Error for RabbitError {}