use serde::de::DeserializeOwned;
use crate::serializer::event_registry::EventRegistry;
use crate::serializer::EventDeserializer;

pub struct SerdeJSONEventDeserializer<'a> {
    registry: &'a EventRegistry
}

impl EventDeserializer for SerdeJSONEventDeserializer<'_> {
    fn deserialize<T: DeserializeOwned>(&self, raw_event: &str) -> T {
        println!("{:?}", self.registry);

        serde_json::from_str(raw_event).expect("TODO: panic message")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
}