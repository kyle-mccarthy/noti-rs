use contact::Contact;
use notification::Notification;
use provider::Provider;
use template::Template;

pub mod channel;
pub mod contact;
pub mod message;
pub mod notification;
pub mod provider;
pub mod template;

#[derive(Default)]
pub struct Notify<'a> {
    templates: template::manager::Manager<'a>,
    notifications: notification::Manager,
    providers: provider::Manager,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Template error: {0:?}")]
    Template(template::Error),
}

impl<'a> Notify<'a> {
    pub fn register_template<N: Notification, T: Template>(
        &mut self,
        template: T,
    ) -> Result<(), Error> {
        let channel = template.channel();
        let template = template
            .register(&mut self.templates)
            .map_err(Error::Template)?;

        if let Some(_old_template) = self.notifications.set_template::<N>(channel, template) {
            // TODO: remove the old/replaced template from the template manager
        }

        Ok(())
    }

    pub fn register_provider<P: Provider>(&mut self, provider: P) {
        self.providers.register(provider)
    }

    pub async fn send<N: Notification>(&self, _to: Contact, _notification: N) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use serde::Serialize;

    use crate::{notification::Notification, template::email::EmailTemplate, Notify};

    pub enum Notifications {
        NewAccountNotification,
    }

    #[derive(Serialize)]
    pub struct NewAccountNotification {
        name: String,
        activation_url: String,
    }

    impl Notification for NewAccountNotification {
        type Id = Notifications;

        fn id() -> Self::Id {
            Notifications::NewAccountNotification
        }
    }

    #[test]
    pub fn test_register_notification() {
        let email_template = EmailTemplate {
            html: indoc! {r#"
                <mjml>
                    <mj-body>
                        <mj-section>
                            <mj-column>
                                <mj-text>Hi, please verify your account by clicking the following link:</mj-text>
                                <mj-text><a href="{{ activation_url }}">{{ activation_url }}</a></mj-text>
                            </mj-column>
                        </mj-section>
                    </mj-body>
                </mjml>
            "#},
            subject: "Example New Account Notification",
            text: Some("Hi, please verify your account by clicking the following link: {{ activation_url }}"),
        };

        let mut notify = Notify::default();
        let result =
            notify.register_template::<NewAccountNotification, EmailTemplate>(email_template);

        assert!(result.is_ok());
    }
}
