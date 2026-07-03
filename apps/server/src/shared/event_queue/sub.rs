use super::Event;
use crate::shared::{Mail, Mailer};

use std::{error::Error, sync::Arc};
use tokio::sync::mpsc::Receiver;

pub struct EventSubscriber {
    receiver: Receiver<Box<dyn Event>>,
    mailer: Arc<Mailer>,
}

impl EventSubscriber {
    pub fn new(receiver: Receiver<Box<dyn Event>>, mailer: Arc<Mailer>) -> Self {
        Self { receiver, mailer }
    }

    pub fn run(self) {
        tokio::spawn(async move {
            if let Err(e) = self.subscribe().await {
                tracing::error!("Error in event subscriber: {e:?}");
            }
        });
    }

    async fn subscribe(mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.recv().await {
            let mailer = Arc::clone(&self.mailer);

            tokio::spawn(async move {
                if let Err(e) = Self::handle(mailer, event).await {
                    tracing::error!("Error handling event: {e:?}");
                }
            });
        }

        Ok(())
    }

    async fn handle(mailer: Arc<Mailer>, event: Box<dyn Event>) -> Result<(), Box<dyn Error>> {
        match event.key() {
            "mailer.mail" => {
                let mail = event.downcast::<Mail>()?;
                mailer.send(mail).await?;
            }
            _ => {}
        }

        Ok(())
    }
}
