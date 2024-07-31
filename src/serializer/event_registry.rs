use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct EventRegistry {
    internal: HashMap<String, TypeId>,
}

impl EventRegistry {
    pub fn new(registry: HashMap<String, TypeId>) -> Self {
        Self {
            internal: registry,
        }
    }

    pub fn add(&mut self, event_name: &'static str, event_type: TypeId) {
        self.internal.insert(event_name.to_string(), event_type);
    }
}