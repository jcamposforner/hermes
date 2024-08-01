use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EventDeserializable<T: Serialize> {
    pub data: EventDeserializableData<T>,
    pub meta: EventDeserializableMeta
}

#[derive(Deserialize, Serialize)]
pub struct EventDeserializableMeta {}

#[derive(Deserialize, Serialize)]
pub struct EventDeserializableData<T: Serialize> {
    #[serde(rename = "type")]
    pub event_name: String,
    pub attributes: T,
}