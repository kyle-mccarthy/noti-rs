use async_trait::async_trait;
use lettre::{
    address::AddressError,
    message::{Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use super::{DynProvider, Error, Provider};
use crate::{channel::EmailChannel, message::email::Email};

pub struct SmtpProvider {
    default_sender: Option<String>,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpProvider {
    pub fn new(transport: AsyncSmtpTransport<Tokio1Executor>) -> Self {
        Self {
            transport,
            default_sender: None,
        }
    }

    pub fn set_default_sender(&mut self, default_sender: String) {
        self.default_sender = Some(default_sender);
    }
}

#[async_trait]
impl Provider for SmtpProvider {
    type Channel = EmailChannel;

    async fn send(&self, message: Email) -> Result<(), Error> {
        let from: Mailbox = message
            .from
            .as_ref()
            .or_else(|| self.default_sender.as_ref())
            .ok_or_else(|| {
                Error::Message(
                    "The message is missing email's sender and provider has no default sender."
                        .to_string(),
                )
            })?
            .parse()
            .map_err(|e: AddressError| Error::Message(e.to_string()))?;

        let to: Mailbox = message
            .to
            .parse()
            .map_err(|e: AddressError| Error::Message(e.to_string()))?;

        let builder = Message::builder()
            .subject(message.contents.subject)
            .to(to)
            .from(from);

        let result = if let Some(text) = message.contents.text {
            builder.multipart(MultiPart::alternative_plain_html(
                text,
                message.contents.html,
            ))
        } else {
            builder.singlepart(SinglePart::html(message.contents.html))
        };

        let email = result.map_err(|e| Error::Message(e.to_string()))?;

        self.transport
            .send(email)
            .await
            .map_err(|e| Error::Send(e.into()))?;

        Ok(())
    }

    fn id(&self) -> &str {
        "smtp-provider"
    }

    fn into_dyn_provider(self) -> DynProvider {
        DynProvider::Email(Box::new(self))
    }
}
