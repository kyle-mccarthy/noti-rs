use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::template::email::RenderedEmailTemplate;

pub mod memory;
pub mod smtp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid email address")]
    EmailAddress(String),

    #[error("Missing sender")]
    MissingSender,

    #[error("Error occured when sending the message")]
    SendFailed(#[source] anyhow::Error),

    #[error("Unknown error occured")]
    Unknown(#[source] anyhow::Error),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Email {
    /// The recipient of the email.
    pub to: String,
    /// The sender of the email. This is optional, but the email provider may
    /// require it if can't accept default senders.
    pub from: Option<String>,
    /// Subject to use for the email.
    pub subject: String,
    /// HTML version of the email.
    pub html: String,
    /// Optional plain text version of the email.
    pub text: Option<String>,
}

#[mockall::automock]
pub trait EmailNotification {
    /// The recipient/who to send the email to
    fn to(&self) -> String;

    /// The sender/who send the email
    fn from(&self) -> Option<String>;

    fn build(&self, rendered: RenderedEmailTemplate) -> Result<Email, Error> {
        Ok(Email {
            to: self.to(),
            from: self.from(),
            subject: rendered.subject,
            html: rendered.html,
            text: rendered.text,
        })
    }
}

#[async_trait]
pub trait EmailProvider {
    fn default_sender(&self) -> Option<String> {
        None
    }

    async fn send(&self, message: Email) -> Result<(), Error>;
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
    struct ActivateAccountNotification {
        email: String,
        first_name: String,
        url: String,
    }

    impl EmailTemplate for ActivateAccountNotification {
        const HTML: &'static str = indoc! {r#"
            <mjml>
                <mj-body>
                    <mj-section>
                        <mj-column>
                            <mj-text>Welcome {{ first_name }}!</mj-text>
                            <mj-text>To get started, <a href="{{ url }}">activate your account</a>!</mj-text>
                        </mj-column>
                    </mj-section>
                </mj-body>
            </mjml>
        "#};
        const SUBJECT: &'static str = "Active Account";
        const TEXT: Option<&'static str> = Some("Hello {{ name }}!");
    }

    impl EmailNotification for ActivateAccountNotification {
        fn to(&self) -> String {
            self.email.clone()
        }

        fn from(&self) -> Option<String> {
            Some("no-reply@test.com".to_string())
        }
    }

    #[test]
    fn test_it_builds_email() {
        let notification = ActivateAccountNotification {
            email: "test@test.com".to_string(),
            first_name: "Test".to_string(),
            url: "https://google.com".to_string(),
        };

        let mut templates = TemplateManager::new();
        templates
            .register::<ActivateAccountNotification>()
            .expect("should register ActivateAccountNotification");

        let rendered = templates
            .render(&notification)
            .expect("should render ActivateAccountNotification");

        let rendered = rendered
            .into_email()
            .expect("should be RenderedEmailTemplate");

        let email = notification.build(rendered);

        assert!(email.is_ok());
    }
}
