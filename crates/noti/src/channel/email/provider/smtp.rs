use async_trait::async_trait;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::{
    channel::{
        email::{Address, Email},
        ChannelType, Error,
    },
    Provider,
};

pub struct SmtpProvider {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpProvider {
    pub fn new(transport: AsyncSmtpTransport<Tokio1Executor>) -> Self {
        Self { transport }
    }
}

impl TryFrom<Address> for Mailbox {
    type Error = Error;

    fn try_from(addr: Address) -> Result<Mailbox, Error> {
        let email = addr
            .email()
            .parse::<lettre::Address>()
            .map_err(|e| Error::InvalidContact {
                source: e.into(),
                context: Some("The emaill address could not be convered into a lettre::Address"),
                channel_type: ChannelType::Email,
            })?;

        let name: Option<String> = addr.name().map(|n| n.to_owned());

        Ok(Mailbox::new(name, email))
    }
}

#[async_trait]
impl Provider for SmtpProvider {
    type Message = Email;

    fn id(&self) -> &'static str {
        "smtp"
    }

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        let from: Mailbox = message.from().clone().try_into()?;

        let to: Mailbox = message.to().clone().try_into()?;

        let builder = Message::builder()
            .subject(message.subject())
            .to(to)
            .from(from);

        let result = if let Some(text) = message.text() {
            builder.multipart(MultiPart::alternative_plain_html(
                text.to_owned(),
                message.html().to_owned(),
            ))
        } else {
            builder.singlepart(SinglePart::html(message.html().to_owned()))
        };

        let email = result.map_err(|e| Error::Send {
            source: e.into(),
            channel_type: ChannelType::Email,
            context: Some("failed to build the lettre::Message"),
            provider_id: self.id(),
        })?;

        self.transport.send(email).await.map_err(|e| Error::Send {
            source: e.into(),
            channel_type: ChannelType::Email,
            context: Some("SMTP email provider failed to send the email"),
            provider_id: self.id(),
        })?;

        Ok(())
    }
}
