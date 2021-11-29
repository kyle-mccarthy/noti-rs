use std::any::TypeId;

use contact::Contact;
use futures::stream::{self, StreamExt, TryStreamExt};
use message::Message;
use notification::Notification;
use provider::{DynProvider, Provider};
use template::{Render, RenderedTemplate, Template};

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

    #[error("The notification hasn't been registered: {0:?}")]
    NotRegistered(TypeId),

    #[error("Failed to create the message")]
    MessageCreation(provider::Error),

    #[error("Provider encountered an error while sending the message")]
    Send(provider::Error),
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

    pub async fn send<N: Notification>(
        &self,
        to: Contact,
        notification: N,
    ) -> Result<usize, Error> {
        let templates = self
            .notifications
            .get_templates(&notification)
            .ok_or_else(|| Error::NotRegistered(notification.type_id()))?;

        // iterate over the notification's templates and render each one
        let message_contents = templates
            .filter_map(|(channel_type, template)| {
                let provider = self.providers.get_provider(*channel_type)?;
                Some((provider, template))
            })
            .map(|(provider, template)| {
                let contents = template
                    .render(&self.templates, &notification)
                    .map_err(Error::Template)?;
                Ok((provider, contents)) as Result<_, Error>
            })
            .collect::<Result<Vec<(&provider::DynProvider, RenderedTemplate)>, Error>>()?;

        // iterate over the rendered templates/message contents, producing a message for
        // each one
        let messages = message_contents
            .into_iter()
            .filter(|(provider, contents)| provider.can_create_message(&to, &contents))
            .map(|(provider, contents)| {
                let message = provider
                    .create_message(&to, contents)
                    .map_err(Error::MessageCreation)?;
                Ok((provider, message))
            })
            .collect::<Result<Vec<(&provider::DynProvider, message::Message)>, Error>>()?;

        // send all the messages
        let out: Result<Vec<()>, Error> = stream::iter(messages.into_iter())
            .then(|(provider, message): (&DynProvider, Message)| async {
                provider.send(message).await.map_err(Error::Send)?;
                Ok(())
            })
            .try_collect()
            .await;

        let messages_sent = out?.len();

        Ok(messages_sent)
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
