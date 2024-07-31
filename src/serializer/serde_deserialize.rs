use std::any::TypeId;
use downcaster::{downcast, downcast_ref};
use serde::de::DeserializeOwned;
use serde_json::Value;
use crate::event::Event;
use crate::serializer::error::DeserializeError;
use crate::serializer::event_registry::EventRegistry;
use crate::serializer::{EventDeserialized, EventDeserializer};

pub struct SerdeJSONEventDeserializer<'a> {
    event_registry: &'a EventRegistry
}

impl<'a> SerdeJSONEventDeserializer<'a> {
    pub fn new(event_registry: &'a EventRegistry) -> Self {
        Self {
            event_registry
        }
    }
}

impl EventDeserializer for SerdeJSONEventDeserializer<'_> {
    fn deserialize(&self, raw_event: &str) -> Result<Box<dyn Event>, DeserializeError> {
        let json_value: Value = serde_json::from_str(raw_event)
            .map_err(|_| DeserializeError::UnableToDeserializeEvent)?;

        let event_name = json_value
            .get("event_name")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string())
            .ok_or(DeserializeError::UnableToDeserializeEvent)?;

        self.event_registry.get(event_name.as_str())
            .map(|event_factory| {
                event_factory(json_value)
            })
            .ok_or(DeserializeError::UnableToDeserializeEvent)?
    }
}

#[cfg(test)]
mod tests {
    use downcaster::Downcast;
    use serde::Deserialize;
    use crate::event::{Event, EventIdentifiable};

    use super::*;

    #[derive(Deserialize, Debug)]
    struct SerializableEvent {
        id: String
    }

    impl EventDeserialized for SerializableEvent {
        fn from_value(value: Value) -> Result<Box<dyn Event>, DeserializeError> {
            let id = value
                .get("payload")
                .and_then(|value| value.get("id"))
                .and_then(|value| value.as_str())
                .map(|value| value.to_string())
                .ok_or(DeserializeError::UnableToDeserializeEvent)?;

            Ok(
                Box::new(
                    Self {
                        id
                    }
                )
            )
        }
    }

    impl Event for SerializableEvent {}

    impl EventIdentifiable for SerializableEvent {
        fn event_name() -> &'static str {
            "serializable_event"
        }
    }

    #[test]
    fn it_should_deserialize() {
        let json = "{\"event_name\":\"serializable_event\",\"payload\":{\"id\":\"1\"}}".to_string();

        let mut registry = EventRegistry::default();
        registry.add::<SerializableEvent>();

        let deserializer = SerdeJSONEventDeserializer::new(&registry);
        let event = deserializer.deserialize(&json);

        let b = event.unwrap();

        let id = (*b).type_id();
        let type_id = TypeId::of::<SerializableEvent>();
        
        match id {
             type_id => {
                 let event = (*b).downcast_ref::<SerializableEvent>();

                 println!("{:?}", event);
            },
            _ => {}
        };
    }
}