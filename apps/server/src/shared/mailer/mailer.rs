use super::{Mail, MailerConfig};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};

pub struct Mailer {
    config: MailerConfig,
    client: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new(config: MailerConfig) -> Self {
        let credentials =
            Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

        let client = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .expect("Failed to create SMTP transport")
            .port(config.smtp_port.parse().expect("Invalid SMTP port"))
            .credentials(credentials)
            .build();

        Self { client, config }
    }

    pub async fn send(&self, mail: &Mail) -> Result<(), Box<dyn std::error::Error>> {
        let email_from_fmt = format!("Ramtun <{}>", self.config.smtp_username);

        let from = email_from_fmt
            .parse::<Mailbox>()
            .inspect_err(|e| eprintln!("Failed to parse sender email address: {e}"))?;

        let to = mail
            .to
            .parse::<Mailbox>()
            .inspect_err(|e| eprintln!("Failed to parse recipient email address: {e}"))?;

        let message = Message::builder()
            .from(from)
            .to(to)
            .subject(mail.subject.clone())
            .header(ContentType::TEXT_HTML)
            .body(mail.html.clone())
            .inspect_err(|e| eprintln!("Failed to build email message: {e}"))?;

        self.client.send(message).await.inspect_err(|e| {
            eprintln!("Failed to send email: {e}");
        })?;

        Ok(())
    }
}
