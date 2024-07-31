use std::any::{Any, TypeId};
use std::collections::HashMap;

use serde_json::Value;

use crate::event::{Event, EventIdentifiable};
use crate::serializer::error::DeserializeError;
use crate::serializer::EventDeserialized;

type SubscriberClosure = Box<fn(Value) -> Result<Box<dyn Event>, DeserializeError>>;

#[derive(Default)]
pub struct EventRegistry {
    internal: HashMap<String, SubscriberClosure>,
}

impl EventRegistry {
    pub fn new(registry: HashMap<String, SubscriberClosure>) -> Self {
        Self {
            internal: registry,
        }
    }

    pub fn add<T: EventIdentifiable + EventDeserialized>(&mut self) {
        self.internal.insert(T::event_name().to_string(), Box::new(T::from_value));
    }

    pub fn get(&self, event_name: &str) -> Option<&SubscriberClosure> {
        self.internal.get(event_name)
    }
}