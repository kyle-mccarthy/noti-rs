use async_trait::async_trait;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use super::{Address, Email, EmailProvider, Error};

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
            .map_err(|e| Error::Address(format!("{:?}", e)))?;

        let name: Option<String> = addr.name().map(|n| n.to_owned());

        Ok(Mailbox::new(name, email))
    }
}

#[async_trait]
impl EmailProvider for SmtpProvider {
    fn id(&self) -> &str {
        "smtp"
    }

    async fn send(&self, message: Email) -> Result<(), Error> {
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

        let email = result.map_err(|e| Error::Message(format!("{:?}", e)))?;

        self.transport
            .send(email)
            .await
            .map_err(|e| Error::Send(format!("{0:?}", e)))?;

        Ok(())
    }
}
