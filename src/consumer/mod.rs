use std::error::Error;
use std::fmt::Display;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serializer::deserialized_event::EventDeserializable;

pub mod rabbitmq_consumer;
pub mod rabbitmq_retryer;

#[allow(async_fn_in_trait)]
pub trait AsyncConsumer {
    async fn consume(&mut self);
}

#[derive(Debug)]
pub enum PayloadHandlerError {
    UnrecoverableError,
    Inner(Box<dyn Error>),
}

impl Display for PayloadHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PayloadHandlerError::UnrecoverableError => write!(f, "Unrecoverable error"),
            PayloadHandlerError::Inner(e) => write!(f, "Inner error: {}", e),
        }
    }
}

impl Error for PayloadHandlerError {}

#[allow(async_fn_in_trait)]
pub trait PayloadHandler<T: Serialize + DeserializeOwned> {
    async fn handle_value_payload(&mut self, payload: &EventDeserializable<T>) -> Result<(), PayloadHandlerError>;
}

#[macro_export]
macro_rules! impl_payload_handler {
    ($struct_name:ident, $(($event_name:literal, $event_type:ident, $method_name:ident)),* )=> {
        $(
            impl_async_event_handler!($struct_name, $method_name, $event_type);
        )*

        impl PayloadHandler<Value> for $struct_name {
            async fn handle_value_payload(&mut self, payload: &EventDeserializable<Value>) -> Result<(), PayloadHandlerError> {
                let attr = payload.data.attributes.clone();
                let event_name = payload.data.event_name.as_str();

                $(
                    if event_name == $event_name {
                        let event = serde_json::from_value::<$event_type>(attr).unwrap();
                        self.$method_name(&event).await;
                        return Ok(());
                    }
                )*

                Err(PayloadHandlerError::UnrecoverableError)
            }
        }
    };
}
