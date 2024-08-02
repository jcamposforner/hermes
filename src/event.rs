use std::collections::HashMap;

use downcaster::AsAny;
#[cfg(feature = "serializer")]
use serde::{Deserialize, Serialize};

pub trait Event: AsAny + Sync + Send + 'static {
    fn event_name(&self) -> &'static str;

    fn event_version(&self) -> &'static str {
        "1.0"
    }
}

pub trait EventWithMetadata: AsAny + Sync + Send + 'static {
    fn add_metadata(&mut self, key: String, value: String);
    fn get_metadata(&self, key: &str) -> Option<&String>;
    fn metadata(&self) -> &EventMetadata;
}

pub trait EventName {
    fn static_event_name() -> &'static str;
}

pub trait DomainEvent: Event {}

#[cfg(feature = "serializer")]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata(HashMap<String, String>);

#[cfg(not(feature = "serializer"))]
#[derive(Default, Debug, Clone)]
pub struct EventMetadata(HashMap<String, String>);

impl EventMetadata {
    pub fn add(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}