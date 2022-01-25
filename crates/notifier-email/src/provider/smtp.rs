use async_trait::async_trait;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use notifier::{provider::Error, Provider};

use crate::{EmailAddress, EmailMessage};

pub struct SmtpProvider {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpProvider {
    pub fn new(transport: AsyncSmtpTransport<Tokio1Executor>) -> Self {
        Self { transport }
    }

    pub async fn send(&self, message: EmailMessage) -> Result<(), Error> {
        let from: Mailbox = message.from().clone().try_into()?;

        let to: Mailbox = message.to().clone().try_into()?;

        let builder = Message::builder()
            .subject(message.contents().subject())
            .to(to)
            .from(from);

        let result = if let Some(text) = message.contents().text() {
            builder.multipart(MultiPart::alternative_plain_html(
                text.to_owned(),
                message.contents().html().to_owned(),
            ))
        } else {
            builder.singlepart(SinglePart::html(message.contents().html().to_owned()))
        };

        let email = result.map_err(|e| Error::Unknown {
            source: e.into(),
            channel_id: "email",
            provider_id: "smtp",
            context: Some("failed to build the lettre::Message"),
        })?;

        self.transport.send(email).await.map_err(|e| Error::Send {
            source: e.into(),
            channel_id: "email",
            provider_id: "smtp",
            context: Some("SMTP email provider failed to send the email"),
        })?;

        Ok(())
    }
}

impl TryFrom<EmailAddress> for Mailbox {
    type Error = Error;

    fn try_from(addr: EmailAddress) -> Result<Mailbox, Error> {
        let email = addr
            .email()
            .parse::<lettre::Address>()
            .map_err(|e| Error::Contact {
                source: e.into(),
                context: Some("The emaill address could not be convered into a lettre::Address"),
                channel_id: "email",
                provider_id: "smtp",
            })?;

        let name: Option<String> = addr.name().map(|n| n.to_owned());

        Ok(Mailbox::new(name, email))
    }
}

#[async_trait]
impl Provider for SmtpProvider {
    type Message = EmailMessage;

    fn id(&self) -> &'static str {
        "smtp"
    }

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        self.send(message).await
    }
}
