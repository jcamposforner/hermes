use serde::{Deserialize, Serialize};

use crate::event::Event;

#[derive(Deserialize, Serialize)]
pub struct EventDeserializable<T: Event + Serialize> {
    pub data: EventDeserializableData<T>,
    pub meta: EventDeserializableMeta
}

#[derive(Deserialize, Serialize)]
pub struct EventDeserializableMeta {}

#[derive(Deserialize, Serialize)]
pub struct EventDeserializableData<T: Event + Serialize> {
    #[serde(rename = "type")]
    pub event_name: String,
    pub attributes: T,
}