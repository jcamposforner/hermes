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
    ($struct_name:ident, $(($event_name:expr, $event_type:ident, $method_name:ident)),* )=> {
        $(
            $crate::impl_async_event_handler!($struct_name, $method_name, $event_type);
        )*

        impl $crate::consumer::PayloadHandler<serde_json::Value> for $struct_name {
            async fn handle_value_payload(&mut self, payload: &$crate::serializer::deserialized_event::EventDeserializable<serde_json::Value>) -> Result<(), $crate::consumer::PayloadHandlerError> {
                let attr = payload.data.attributes.clone();
                let event_name = payload.data.event_name.as_str();

                $(
                    if event_name == $event_name {
                        let event = serde_json::from_value::<$event_type>(attr).unwrap();
                        self.$method_name(&event).await;
                        return Ok(());
                    }
                )*

                Err($crate::consumer::PayloadHandlerError::UnrecoverableError)
            }
        }
    };
    ($struct_name:ident, $(($event_type:ident, $method_name:ident)),* )=> {
        $(
            $crate::impl_async_event_handler!($struct_name, $method_name, $event_type);
        )*

        impl $crate::consumer::PayloadHandler<serde_json::Value> for $struct_name {
            async fn handle_value_payload(&mut self, payload: &$crate::serializer::deserialized_event::EventDeserializable<serde_json::Value>) -> Result<(), $crate::consumer::PayloadHandlerError> {
                let attr = payload.data.attributes.clone();
                let event_name = payload.data.event_name.as_str();

                $(
                    if event_name == <$event_type as $crate::event::EventName>::static_event_name() {
                        let event = serde_json::from_value::<$event_type>(attr).unwrap();
                        self.$method_name(&event).await;
                        return Ok(());
                    }
                )*

                Err($crate::consumer::PayloadHandlerError::UnrecoverableError)
            }
        }
    };
    ($struct_name:ident, $(($event_name:expr, $event_type:ident)),* )=> {
        $(
            $crate::impl_async_event_handler!($struct_name, $event_type);
        )*

        impl $crate::consumer::PayloadHandler<serde_json::Value> for $struct_name {
            async fn handle_value_payload(&mut self, payload: &$crate::serializer::deserialized_event::EventDeserializable<serde_json::Value>) -> Result<(), $crate::consumer::PayloadHandlerError> {
                let attr = payload.data.attributes.clone();
                let event_name = payload.data.event_name.as_str();

                $(
                    if event_name == $event_name {
                        let event = serde_json::from_value::<$event_type>(attr).unwrap();
                        self.on(&event).await;
                        return Ok(());
                    }
                )*

                Err($crate::consumer::PayloadHandlerError::UnrecoverableError)
            }
        }
    };
}
