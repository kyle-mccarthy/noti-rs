use async_trait::async_trait;
use lettre::{
    error::Error as EmailError,
    message::{Mailbox, MultiPart, SinglePart},
    Message,
};

use super::Error;
use crate::template::email::RenderedEmailTemplate;

impl From<EmailError> for Error {
    fn from(source: EmailError) -> Self {
        Self::Message(source.into())
    }
}

pub trait EmailNotification {
    /// The recipient/who to send the email to
    fn to(&self) -> &str;

    /// The sender/who send the email
    fn from(&self) -> &str;

    /// Optional address to use for the reply to
    fn reply_to(&self) -> Option<&str> {
        None
    }

    fn build(&self, rendered: RenderedEmailTemplate) -> Result<Message, Error> {
        let to: Mailbox = self.to().parse()?;
        let from: Mailbox = self.from().parse()?;

        let mut builder = Message::builder()
            .subject(rendered.subject)
            .to(to)
            .from(from);

        if let Some(reply_to) = self.reply_to() {
            let reply_to: Mailbox = reply_to.parse()?;
            builder = builder.reply_to(reply_to);
        }

        if let Some(text) = rendered.text {
            Ok(builder.multipart(MultiPart::alternative_plain_html(text, rendered.html))?)
        } else {
            Ok(builder.singlepart(SinglePart::html(rendered.html))?)
        }
    }
}

#[async_trait]
pub trait EmailProvider {
    async fn send(&self, message: Message) -> Result<(), Error>;
}

#[cfg(test)]
mod test_email_channel {
    use indoc::indoc;
    use serde::Serialize;

    use super::EmailNotification;
    use crate::{
        template::{email::EmailTemplate, TemplateManager},
        EmailNotification,
    };

    #[derive(Serialize, EmailNotification)]
    struct ActivateEmailNotification {
        email: String,
        first_name: String,
        url: String,
    }

    impl EmailTemplate for ActivateEmailNotification {
        const HTML: &'static str = indoc! {"
            <mjml>
                <mj-body>
                    <mj-section>
                        <mj-column>
                            <mj-text>Hello {{ name }}!</mj-text>
                        </mj-column>
                    </mj-section>
                </mj-body>
            </mjml>
        "};
        const SUBJECT: &'static str = "Hello {{ name }}!";
        const TEXT: Option<&'static str> = Some("Hello {{ name }}!");
    }

    impl EmailNotification for ActivateEmailNotification {
        fn to(&self) -> &str {
            &self.email
        }

        fn from(&self) -> &str {
            "no-reply@test.com"
        }
    }

    #[test]
    fn test_it_builds_email() {
        let notification = ActivateEmailNotification {
            email: "test@test.com".to_string(),
            first_name: "Test".to_string(),
            url: "https://google.com".to_string(),
        };

        let mut templates = TemplateManager::new();
        templates
            .register::<ActivateEmailNotification>()
            .expect("should register ActivateEmailNotification");

        let rendered = templates
            .render(&notification)
            .expect("should render ActivateEmailNotification");

        let rendered = rendered
            .into_email()
            .expect("should be RenderedEmailTemplate");

        let email = notification.build(rendered);

        assert!(email.is_ok());
    }
}
