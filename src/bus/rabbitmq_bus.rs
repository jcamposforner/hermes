use std::sync::Arc;

use serde::Serialize;

use crate::bus::AsynchronousEventBus;
use crate::bus::error::PublishError;
use crate::event::{Event, EventWithMetadata};
use crate::rabbit::rabbit_publisher::RabbitPublisher;
use crate::serializer::EventSerializer;

pub struct RabbitEventBus<'a, T: EventSerializer> {
    serializer: &'a T,
    exchange: String,
    publisher: Arc<RabbitPublisher>
}

impl<'a, T: EventSerializer> RabbitEventBus<'a, T> {
    pub async fn new(
        publisher: Arc<RabbitPublisher>,
        serializer: &'a T,
        exchange: String
    ) -> Self {
        Self {
            serializer,
            exchange,
            publisher
        }
    }
}

impl<T: EventSerializer> AsynchronousEventBus for RabbitEventBus<'_, T> {
    async fn publish<E: Event + EventWithMetadata + Serialize>(&self, event: E) -> Result<(), PublishError> {
        let payload = self.serializer.serialize(&event).map_err(|_| PublishError::CannotSerializeEvent)?;

        self.publisher.publish(payload.as_bytes(), event.event_name(), self.exchange.as_str()).await
    }
}
