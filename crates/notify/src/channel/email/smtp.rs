use async_trait::async_trait;
use lettre::{
    address::AddressError,
    message::{Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use super::{Email, EmailProvider, Error};

impl From<AddressError> for Error {
    fn from(source: AddressError) -> Self {
        Self::EmailAddress(source.to_string())
    }
}

#[derive(notify_macros::EmailProvider)]
pub struct SmtpTransport {
    default_sender: Option<String>,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

#[async_trait]
impl EmailProvider for SmtpTransport {
    fn default_sender(&self) -> Option<String> {
        self.default_sender.clone()
    }

    async fn send(&self, message: Email) -> Result<(), Error> {
        let to: Mailbox = message.to.parse()?;

        let from: Mailbox = message
            .from
            .or_else(|| self.default_sender())
            .ok_or(Error::MissingSender)?
            .parse()?;

        let builder = Message::builder()
            .subject(message.subject)
            .to(to)
            .from(from);

        let result = if let Some(text) = message.text {
            builder.multipart(MultiPart::alternative_plain_html(text, message.html))
        } else {
            builder.singlepart(SinglePart::html(message.html))
        };

        let message = result.map_err(|source| Error::Unknown(source.into()))?;

        self.transport
            .send(message)
            .await
            .map_err(|e| Error::SendFailed(e.into()))?;

        Ok(())
    }
}
