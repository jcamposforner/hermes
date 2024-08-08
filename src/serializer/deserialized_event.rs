use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{DeserializeOwned, MapAccess, Visitor};

#[derive(Serialize)]
pub struct EventDeserializable<T: Serialize> {
    pub data: EventDeserializableData<T>,
}

#[derive(Deserialize, Serialize)]
pub struct EventDeserializableData<T: Serialize> {
    #[serde(rename = "type")]
    pub event_name: String,
    pub attributes: T,
}

struct EventDeserializableVisitor<T: Serialize> {
    marker: std::marker::PhantomData<T>,
}

impl<'de, T: Serialize + DeserializeOwned> Visitor<'de> for EventDeserializableVisitor<T> {
    type Value = EventDeserializable<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct EventDeserializable")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                                      where
                                          V: MapAccess<'de>,
    {
        let mut data: Option<serde_json::Map<String, serde_json::Value>> = None;
        let mut meta: Option<serde_json::Value> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "data" => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("type"));
                    }
                    data = Some(map.next_value()?);
                }
                "meta" => {
                    if meta.is_some() {
                        return Err(de::Error::duplicate_field("meta"));
                    }
                    meta = Some(map.next_value()?);
                }
                _ => {
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }

        let mut data = data.ok_or(de::Error::missing_field("data"))?;
        let attributes = data.get_mut("attributes").ok_or(de::Error::missing_field("attributes"))?;
        if let serde_json::Value::Object(attributes) = attributes {
            attributes.insert("metadata".to_string(), meta.ok_or(de::Error::missing_field("meta"))?);
        }

        let attributes_json = serde_json::Value::Object(data);
        let attributes: EventDeserializableData<T> = serde_json::from_value(attributes_json).map_err(de::Error::custom)?;

        Ok(
            EventDeserializable {
                data: attributes,
            }
        )
    }
}

impl<'de, T: Serialize + DeserializeOwned> Deserialize<'de> for EventDeserializable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                                       where
                                           D: Deserializer<'de>,
    {
        deserializer.deserialize_map(EventDeserializableVisitor {
            marker: std::marker::PhantomData,
        })
    }
}