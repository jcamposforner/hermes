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
    fn drain_metadata(&mut self) -> EventMetadata;
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

#[macro_export]
macro_rules! event {
    ($event_name:ident,$($field_name:ident:$field_type:ty),*) => {
        #[derive(Debug, Clone)]
        pub struct $event_name {
            pub metadata: hermes::event::EventMetadata,
            $(
                pub $field_name: $field_type,
            )*
        }

        impl $event_name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    metadata: hermes::event::EventMetadata::default(),
                    $($field_name,)*
                }
            }
        }

        impl hermes::event::Event for $event_name {
            fn event_name(&self) -> &'static str {
                stringify!($event_name)
            }

            fn event_version(&self) -> &'static str {
                "1.0"
            }
        }

        $crate::event_metadata!($event_name);
    };
    ($event_name:ident,$event_trait_name:literal,$($field_name:ident:$field_type:ty),*) => {
        #[derive(Debug, Clone)]
        pub struct $event_name {
            pub metadata: hermes::event::EventMetadata,
            $(
                pub $field_name: $field_type,
            )*
        }

        impl hermes::event::Event for $event_name {
            fn event_name(&self) -> &'static str {
                $event_trait_name
            }

            fn event_version(&self) -> &'static str {
                "1.0"
            }
        }

        $crate::event_metadata!($event_name);
    };
}

#[macro_export]
macro_rules! event_metadata {
    ($event_name:ident) => {
        impl hermes::event::EventWithMetadata for $event_name {
            fn add_metadata(&mut self, key: String, value: String) {
                self.metadata.add(key, value);
            }

            fn get_metadata(&self, key: &str) -> Option<&String> {
                self.metadata.get(key)
            }

            fn metadata(&self) -> &hermes::event::EventMetadata {
                &self.metadata
            }

            fn drain_metadata(&mut self) -> hermes::event::EventMetadata {
                std::mem::take(&mut self.metadata)
            }
        }
    };
}