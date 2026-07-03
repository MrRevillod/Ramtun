mod r#pub;
mod sub;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{any::Any, error::Error, fmt::Debug};
use sword::prelude::*;

use crate::shared::AppError;

#[derive(Clone, Debug, Deserialize)]
#[config(key = "event-queue")]
pub struct EventQueueConfig {
    pub buffer_size: usize,
    pub num_of_event_retry: u8,
    pub delay_between_event_retry_ms: u64,
}

pub trait Event: Send + Sync + Any + Debug {
    fn key(&self) -> &'static str;

    fn timestamp(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl dyn Event {
    pub fn downcast<T: Event>(&self) -> Result<&T, Box<dyn Error>> {
        let downcasted = (self as &dyn Any)
            .downcast_ref::<T>()
            .ok_or(AppError::InternalError)?;

        Ok(downcasted)
    }
}
