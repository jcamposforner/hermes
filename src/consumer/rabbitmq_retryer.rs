use std::sync::Arc;

use lapin::message::Delivery;
use lapin::types::FieldTable;

use crate::bus::error::PublishError;
use crate::rabbit::rabbit_publisher::RabbitPublisher;

pub struct RabbitMQRetryer {
    pub max_retries: u32,
    publisher: Arc<RabbitPublisher>,
}

impl RabbitMQRetryer {
    pub fn new(publisher: Arc<RabbitPublisher>, max_retries: u32) -> Self {
        RabbitMQRetryer { publisher, max_retries }
    }

    pub async fn retry(&self, delivery: &Delivery, queue_name: &str) -> Result<(), PublishError> {
        let redelivery_count = Self::get_redelivery_count(delivery);
        let exchange = self.get_target_exchange(delivery, redelivery_count);
        let headers = Self::add_redelivery_count_header(delivery, redelivery_count);

        self.publisher.publish_with_headers(
            &delivery.data,
            queue_name,
            &exchange,
            headers
        ).await
    }

    fn get_target_exchange(&self, delivery: &Delivery, redelivery_count: i64) -> String {
        if redelivery_count > self.max_retries as i64 {
            return format!("dead_letter-{}", delivery.exchange);
        }

        format!("retry-{}", delivery.exchange)
    }

    fn add_redelivery_count_header(delivery: &Delivery, redelivery_count: i64) -> FieldTable {
        let mut headers = delivery.properties.headers().clone().unwrap_or_default();
        headers.insert(
            "redelivery_count".into(),
            redelivery_count.into()
        );
        headers
    }

    fn get_redelivery_count(delivery: &Delivery) -> i64 {
        let mut redelivery_count: i64 = 0;
        if let Some(headers) = delivery.properties.headers() {
            if let Some(redelivery_count_value) = headers.inner().get("redelivery_count") {
                if let Some(count) = redelivery_count_value.as_long_long_int() {
                    redelivery_count = count;
                }
            }
        }

        redelivery_count += 1;
        redelivery_count
    }
}