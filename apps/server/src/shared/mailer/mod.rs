mod mailer;

use crate::shared::Event;

use bon::Builder;
use serde::Deserialize;
use sword::prelude::*;

pub use mailer::*;

#[derive(Debug, Clone, Builder)]
pub struct Mail {
    pub to: String,
    pub subject: String,
    pub html: String,
}

impl Event for Mail {
    fn key(&self) -> &'static str {
        "mailer.mail"
    }
}

#[config(key = "mailer")]
#[derive(Debug, Clone, Deserialize)]
pub struct MailerConfig {
    pub smtp_host: String,
    pub smtp_port: String,
    pub smtp_username: String,
    pub smtp_password: String,
}
