use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::event::{Event, EventWithMetadata};
use crate::serializer::{EventDeserializer, EventSerializer};
use crate::serializer::deserialized_event::EventDeserializable;
use crate::serializer::error::{DeserializeError, SerializeError};
use crate::serializer::serialized_event::{EventSerializable, EventSerializableData};

pub struct SerdeJSONEventFormatter;

impl EventSerializer for SerdeJSONEventFormatter {
    fn serialize<T: Event + EventWithMetadata + Serialize>(&self, event: &T) -> Result<String, SerializeError> {
        let event_serializable = EventSerializable::new(
            EventSerializableData::new(event.event_name(), event),
            event.metadata()
        );

        serde_json::to_string(&event_serializable)
            .map_err(|_| SerializeError::UnableToSerializeEvent)
    }
}

impl EventDeserializer for SerdeJSONEventFormatter {
    fn deserialize<T: DeserializeOwned + Serialize>(&self, raw_event: String) -> Result<EventDeserializable<T>, DeserializeError> {
        serde_json::from_str::<EventDeserializable<T>>(&raw_event)
            .map_err(|_| DeserializeError::UnableToDeserializeEvent)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use serde::Deserialize;

    use crate::event::EventMetadata;

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct SerializableEvent {
        id: String,
        metadata: EventMetadata
    }

    impl Event for SerializableEvent {
        fn event_name(&self) -> &'static str {
            "serializable_event"
        }
    }

    impl EventWithMetadata for SerializableEvent {
        fn add_metadata(&mut self, key: String, value: String) {
            self.metadata.add(key, value);
        }

        fn get_metadata(&self, key: &str) -> Option<&String> {
            self.metadata.get(key)
        }

        fn metadata(&self) -> &EventMetadata {
            &self.metadata
        }

        fn drain_metadata(&mut self) -> EventMetadata {
            mem::take(&mut self.metadata)
        }
    }
    
    #[test]
    fn it_should_serialize_event_and_add_event_name() {
        let event = SerializableEvent { id: "1".to_string(), metadata: EventMetadata::default() };
        let serialized = SerdeJSONEventFormatter.serialize(&event);

        assert_eq!(serialized.unwrap(), "{\"data\":{\"type\":\"serializable_event\",\"attributes\":{\"id\":\"1\"}},\"meta\":{}}")
    }

    #[test]
    fn it_should_deserialize_event_and_add_event_name() {
        let json = "{\"data\":{\"type\":\"serializable_event\",\"attributes\":{\"id\":\"1\"}},\"meta\":{}}".to_string();

        let deserialized = SerdeJSONEventFormatter.deserialize::<SerializableEvent>(json);


        assert!(deserialized.is_ok());
        let deserializable = deserialized.unwrap();

        assert_eq!(deserializable.data.event_name, "serializable_event");
        assert_eq!(deserializable.data.attributes.id, "1");
    }

    #[test]
    fn it_should_not_deserialize_event_when_json_is_not_equals() {
        let json = "{\"data\":{\"type\":\"serializable_event\",\"attributes\":{\"idd\":\"1\"}},\"meta\":{}}".to_string();

        let deserialized = SerdeJSONEventFormatter.deserialize::<SerializableEvent>(json);


        assert!(deserialized.is_err());
    }
}