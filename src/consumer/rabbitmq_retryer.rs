use lapin::message::Delivery;

pub struct RabbitMQRetryer {
    pub max_retries: u32,
}

impl RabbitMQRetryer {
    pub fn new(max_retries: u32) -> Self {
        RabbitMQRetryer { max_retries }
    }

    pub async fn retry(&self, delivery: &Delivery) -> bool {
        true
    }
}