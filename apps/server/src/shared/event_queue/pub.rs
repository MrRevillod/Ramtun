use sword::prelude::*;
use tokio::sync::mpsc::Sender;

use super::Event;

#[injectable(provider)]
pub struct EventPublisher {
    sender: Sender<Box<dyn Event>>,
}

impl EventPublisher {
    pub const fn new(sender: Sender<Box<dyn Event>>) -> Self {
        Self { sender }
    }

    pub async fn publish(&self, event: Box<dyn Event>) {
        if let Err(e) = self.sender.send(event).await {
            tracing::error!("Error publishing event: {e}");
        }
    }
}
