use async_trait::async_trait;

use crate::message::email::Email;

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

#[async_trait]
pub trait EmailProvider {
    fn default_sender(&self) -> Option<String> {
        None
    }

    async fn send(&self, message: Email) -> Result<(), Error>;
}

#[cfg(test)]
mod test_utils {
    use indoc::indoc;
    use serde::Serialize;

    use crate::{
        notification::EmailNotification, template::email::EmailTemplate, EmailNotification, Notify,
    };

    #[derive(Serialize, EmailNotification)]
    pub struct ActivateAccountNotification {
        pub email: String,
        pub first_name: String,
        pub url: String,
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
        const SUBJECT: &'static str = "Activate Your Account";
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

    pub fn create_instance() -> Notify<'static> {
        let mut instance = Notify::default();

        instance
            .register_template::<ActivateAccountNotification>()
            .expect("failed to register the template");

        instance
    }
}

#[cfg(test)]
mod test_email_channel {
    use super::test_utils::ActivateAccountNotification;
    use crate::{notification::EmailNotification, template::TemplateManager};

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
    }
}
