use serde::Serialize;
use crate::event::{Event, EventIdentifiable};
use crate::serializer::{EventSerializable, EventSerializer};
use crate::serializer::error::SerializeError;

pub struct SerdeJSONEventSerializer;

impl EventSerializer for SerdeJSONEventSerializer {
    fn serialize<T: Event + EventIdentifiable + Serialize>(&self, event: &T) -> Result<String, SerializeError> {
        let event_serializable = EventSerializable::new(T::event_name(), event);

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
    fn serialize_event() {
        let event = SerializableEvent { id: "1".to_string() };

        let serialized = SerdeJSONEventSerializer.serialize(&event);
        
        println!("{:?}", serialized.unwrap());
    }
}