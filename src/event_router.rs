use std::collections::HashMap;

use serde_json::Value;

pub trait EventHandler {
    fn handle(&mut self, payload: Value);
}

#[derive(Default)]
pub struct EventRouter {
    registry: HashMap<String, Box<dyn EventHandler>>
}

impl EventRouter {
    pub fn new(registry: HashMap<String, Box<dyn EventHandler>>) -> Self {
        Self {
            registry
        }
    }

    pub fn register(&mut self, event_name: String, handler: Box<dyn EventHandler>) {
        self.registry.insert(event_name, handler);
    }

    pub fn route(&mut self, event_name: &str, payload: Value) {
        if let Some(handler) = self.registry.get_mut(event_name) {
            handler.handle(payload);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    struct EventNameHandler;

    impl EventHandler for EventNameHandler {
        fn handle(&mut self, payload: Value) {
            assert_eq!(payload, json!({"key": "value"}));
        }
    }

    #[test]
    fn it_should_route_event() {
        let mut router = EventRouter::default();
        let event_name = "event_name".to_string();
        let payload = json!({"key": "value"});

        let handler = Box::new(EventNameHandler);

        router.register(event_name.clone(), handler);
        router.route(&event_name, payload);
    }
}