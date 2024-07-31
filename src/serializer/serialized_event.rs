use serde::Serialize;
use crate::event::Event;

#[derive(Serialize)]
pub struct EventSerializable<'a, T: Event + Serialize> {
    data: EventSerializableData<'a, T>,
    meta: EventSerializableMeta
}

impl<'a, T: Event + Serialize> EventSerializable<'a, T> {
    pub fn new(data: EventSerializableData<'a, T>, meta: EventSerializableMeta) -> Self {
        Self {
            data,
            meta
        }
    }
}

#[derive(Serialize)]
pub struct EventSerializableMeta {}

#[derive(Serialize)]
pub struct EventSerializableData<'a, T: Event + Serialize> {
    #[serde(rename= "type")]
    event_name: &'a str,
    attributes: &'a T,
}

impl<'a, T: Event + Serialize> EventSerializableData<'a, T> {
    pub fn new(event_name: &'a str, attributes: &'a T) -> Self {
        Self {
            event_name,
            attributes
        }
    }
}