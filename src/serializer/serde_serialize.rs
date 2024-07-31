use serde::Serialize;

use crate::event::{Event, EventIdentifiable};
use crate::serializer::error::SerializeError;
use crate::serializer::EventSerializer;
use crate::serializer::serialized_event::{EventSerializable, EventSerializableData, EventSerializableMeta};

pub struct SerdeJSONEventSerializer;

impl EventSerializer for SerdeJSONEventSerializer {
    fn serialize<T: Event + EventIdentifiable + Serialize>(&self, event: &T) -> Result<String, SerializeError> {
        let event_serializable = EventSerializable::new(
            EventSerializableData::new(T::event_name(), event),
            EventSerializableMeta {}
        );

        serde_json::to_string(&event_serializable)
            .map_err(|_| SerializeError::UnableToSerializeEvent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct SerializableEvent {
        id: String
    }

    impl Event for SerializableEvent {}

    impl EventIdentifiable for SerializableEvent {
        fn event_name() -> &'static str {
            "serializable_event"
        }
    }

    #[test]
    fn it_should_serialize_event_and_add_event_name() {
        let event = SerializableEvent { id: "1".to_string() };
        let serialized = SerdeJSONEventSerializer.serialize(&event);

        assert_eq!(serialized.unwrap(), "{\"data\":{\"type\":\"serializable_event\",\"attributes\":{\"id\":\"1\"}},\"meta\":{}}")
    }
}