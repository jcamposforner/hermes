use std::sync::Arc;

use lapin::{Channel, Connection};
use tokio::sync::{RwLock, RwLockReadGuard};

use crate::rabbit::RabbitError;

pub struct RabbitChannel {
    connection: Arc<Connection>,
    channel: Arc<RwLock<Channel>>,
}

impl RabbitChannel {
    pub fn new(connection: Arc<Connection>, channel: Channel) -> Self {
        Self {
            connection,
            channel: Arc::new(RwLock::new(channel)),
        }
    }

    async fn recreate_channel(&self) -> Result<(), RabbitError> {
        let mut write_guard = self.channel.write().await;
        let channel = self.connection.create_channel()
                          .await
                          .map_err(|_| RabbitError::CannotOpenChannel)?;

        *write_guard = channel;

        drop(write_guard);

        Ok(())
    }

    pub async fn get_guard_channel(&self) -> Result<RwLockReadGuard<Channel>, RabbitError> {
        let channel = self.channel.read().await;

        let is_connected = channel.status().connected();
        if !is_connected {
            drop(channel);
            self.recreate_channel().await?;
        }

        let read_guard = self.channel.read().await;
        Ok(read_guard)
    }
}