use lapin::{Channel, Connection, ExchangeKind};
use lapin::options::{ExchangeDeclareOptions, QueueBindOptions};
use lapin::types::{AMQPValue, FieldTable};

pub struct RabbitConfigurer {
    connection: Connection,
    exchange: String,
    retry_ttl: u64,
}

impl RabbitConfigurer {
    pub fn new(
        connection: Connection,
        exchange: String,
        retry_ttl: u64,
    ) -> Self {
        RabbitConfigurer { connection, exchange, retry_ttl }
    }

    pub async fn configure(&self, queue: (&str, &[&str])) {
        let channel = self.connection.create_channel().await.expect("Cannot open channel");
        self.declare_exchanges(&channel).await;

        self.create_queue(queue.0, queue.1, &channel).await;
        self.create_retry_queue(queue.0, &channel).await;
        self.create_dead_letter_queue(queue.0, &channel).await;
    }

    async fn declare_exchanges(&self, channel: &Channel) {
        let options = ExchangeDeclareOptions {
            durable: true,
            ..Default::default()
        };

        channel.exchange_declare(
            self.exchange.as_str(),
            ExchangeKind::Topic,
            options,
            FieldTable::default()
        ).await.expect("Cannot declare exchange");

        channel.exchange_declare(
            format!("retry-{}", self.exchange.as_str()).as_str(),
            ExchangeKind::Topic,
            options,
            FieldTable::default()
        ).await.expect("Cannot declare retry exchange");

        channel.exchange_declare(
            format!("dead_letter-{}", self.exchange.as_str()).as_str(),
            ExchangeKind::Topic,
            options,
            FieldTable::default()
        ).await.expect("Cannot declare dead_letter exchange");
    }

    async fn create_queue(&self, queue_name: &str, routing_keys: &[&str], channel: &Channel) {
        channel.queue_declare(
            queue_name,
            lapin::options::QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default()
        ).await.expect("Cannot declare queue");

        for routing_key in routing_keys {
            channel.queue_bind(
                queue_name,
                self.exchange.as_str(),
                routing_key,
                Default::default(),
                Default::default()
            ).await.expect("Cannot bind queue to exchange");
        }

        channel.queue_bind(
            queue_name,
            self.exchange.as_str(),
            queue_name,
            Default::default(),
            Default::default()
        ).await.expect("Cannot bind queue to exchange");
    }

    async fn create_retry_queue(&self, queue_name: &str, channel: &Channel) {
        let mut arguments = FieldTable::default();
        arguments.insert("x-dead-letter-exchange".into(), AMQPValue::LongString(self.exchange.as_str().into()));
        arguments.insert("x-dead-letter-routing-key".into(), AMQPValue::LongString(queue_name.into()));
        arguments.insert("x-message-ttl".into(), AMQPValue::LongLongInt(self.retry_ttl as i64));

        channel.queue_declare(
            format!("retry.{}", queue_name).as_str(),
            lapin::options::QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            arguments.clone(),
        ).await.expect("Cannot declare queue");

        channel.queue_bind(
            format!("retry.{}", queue_name).as_str(),
            format!("retry-{}", self.exchange.as_str()).as_str(),
            queue_name,
            QueueBindOptions::default(),
            FieldTable::default()
        ).await.expect("Cannot bind queue to exchange");
    }

    async fn create_dead_letter_queue(&self, queue_name: &str, channel: &Channel) {
        channel.queue_declare(
            format!("dead_letter.{}", queue_name).as_str(),
            lapin::options::QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            Default::default()
        ).await.expect("Cannot declare queue");

        channel.queue_bind(
            format!("dead_letter.{}", queue_name).as_str(),
            format!("dead_letter-{}", self.exchange.as_str()).as_str(),
            queue_name,
            QueueBindOptions::default(),
            Default::default()
        ).await.expect("Cannot bind queue to exchange");
    }
}