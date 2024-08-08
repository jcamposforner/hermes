use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::event::{Event, EventMetadata};

#[derive(Serialize)]
pub struct EventSerializable<'a, T: Event + Serialize> {
    data: EventSerializableData<'a, T>,
    meta: &'a EventMetadata
}

impl<'a, T: Event + Serialize> EventSerializable<'a, T> {
    pub fn new(data: EventSerializableData<'a, T>, meta: &'a EventMetadata) -> Self {
        Self {
            data,
            meta
        }
    }
}

#[derive(Serialize)]
pub struct EventSerializableMeta {}

pub struct EventSerializableData<'a, T: Event + Serialize> {
    event_name: &'a str,
    attributes: &'a T,
}

const SKIP_SERIALIZATION_FIELD: &str = "metadata";

impl<'a, T: Event + Serialize> Serialize for EventSerializableData<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                          where
                                              S: serde::Serializer
    {
        let mut state = serializer.serialize_struct("EventSerializableData", 2)?;
        state.serialize_field("type", &self.event_name)?;

        let mut map = serde_json::Map::new();
        let attributes_json = serde_json::to_value(self.attributes).map_err(serde::ser::Error::custom)?;
        if let serde_json::Value::Object(obj) = attributes_json {
            for (k, v) in obj.into_iter() {
                if k == SKIP_SERIALIZATION_FIELD {
                    continue;
                }

                map.insert(k, v);
            }
        }

        state.serialize_field("attributes", &map)?;

        state.end()
    }
}

impl<'a, T: Event + Serialize> EventSerializableData<'a, T> {
    pub fn new(event_name: &'a str, attributes: &'a T) -> Self {
        Self {
            event_name,
            attributes
        }
    }
}