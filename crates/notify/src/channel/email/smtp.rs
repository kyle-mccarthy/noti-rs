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

impl SmtpTransport {
    pub fn new(transport: AsyncSmtpTransport<Tokio1Executor>) -> Self {
        Self {
            transport,
            default_sender: None,
        }
    }

    pub fn set_default_sender(&mut self, default_sender: &str) -> Option<String> {
        self.default_sender.replace(default_sender.to_string())
    }
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

        let response = self
            .transport
            .send(message)
            .await
            .map_err(|e| Error::SendFailed(e.into()))?;

        dbg!(response);

        Ok(())
    }
}

#[cfg(test)]
mod test_smtp_transport {
    use lettre::{
        transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor,
    };
    use tokio::runtime::Runtime;

    use super::{
        super::test_utils::{create_instance, ActivateAccountNotification},
        SmtpTransport,
    };

    const TEST_SMTP_USER: &str = "shania.runolfsdottir72@ethereal.email";
    const TEST_SMTP_PASS: &str = "vxCUzUZZXEGJ2XtJPB";
    const TEST_SMTP_HOST: &str = "smtp.ethereal.email";

    #[test]
    fn test_create_smtp_transport() {
        // this tests panics if called using tokio::test. lettre has an async call in
        // the smtp pools drop impl.

        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let creds = Credentials::new(TEST_SMTP_USER.to_string(), TEST_SMTP_PASS.to_string());

            let mailer =
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(TEST_SMTP_HOST).unwrap();
            let mailer = mailer
                .credentials(creds)
                .port(587)
                .build::<Tokio1Executor>();

            let channel = SmtpTransport::new(mailer);

            let mut instance = create_instance();
            instance.register_provider(channel);

            let notification = ActivateAccountNotification {
                first_name: "test".to_string(),
                email: "test@test.com".to_string(),
                url: "https://example.com/activate?token=123".to_string(),
            };

            instance.send(notification).await.expect("send failed");
        });
    }
}
