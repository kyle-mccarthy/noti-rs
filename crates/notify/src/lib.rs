use channel::ChannelType;
use contact::Contact;
use email::EmailChannel;
use id::Id;

pub mod channel;
pub mod contact;
pub mod email;
pub mod id;
pub mod notification;
pub mod template;

pub use notification::Notification;
use template::Renderable;

pub trait RegisterTemplate {
    // register the template with the Notify instance
    fn register<N: Id>(
        self,
        notification_id: N,
        notifications: &mut Notify<N>,
    ) -> Result<(), Error>;
}

pub trait RegisterChannel {
    fn register<N: Id>(self, instance: &mut Notify<N>);
}

#[async_trait::async_trait]
pub trait Notifier {
    type Contact;
    type MessageBuilder;

    async fn notify(&self, to: Self::Contact, builder: Self::MessageBuilder) -> Result<(), Error>;
}

#[derive(Default)]
struct Channels {
    email: Option<EmailChannel>,
}

#[derive(Default)]
pub struct Notify<'a, N: Id> {
    templates: template::Engine<'a>,
    notifications: notification::Store<N>,
    channels: Channels,
    // contacts: Option<Box<dyn ContactRepository<Id = ContactId>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Email error: {0}")]
    Email(#[from] email::Error),

    #[error("Template error: {0}")]
    Template(#[from] template::Error),

    #[error("The channel has not been registered. (channel type = {0})")]
    MissingChannel(ChannelType),

    #[error("The notification has not been registered. (id = {0})")]
    UnknownNotification(String),

    #[error("The notification is missing a template for the channel type.")]
    MissingTemplate {
        context: &'static str,
        notification_id: String,
        channel_type: ChannelType,
    },
}

impl<'a, N: Id> Notify<'a, N> {
    /// Register the template for the notification
    pub fn register_template(
        &mut self,
        notification_id: N,
        template: impl RegisterTemplate,
    ) -> Result<(), Error> {
        template.register(notification_id, self)
    }

    /// Register a notification channel
    pub fn register_channel(&mut self, provider: impl RegisterChannel) {
        provider.register(self)
    }

    pub fn send<M: Notification<Id = N>>(&self, _notification: M) -> Result<(), Error> {
        Ok(())
    }

    pub async fn send_now<M: Notification<Id = N>>(
        &self,
        to: Contact,
        notification: M,
    ) -> Result<(), Error> {
        let composite = self.notifications.get(M::id());

        if composite.is_none() {
            return Err(Error::UnknownNotification(M::id().to_string()));
        }

        let composite = composite.unwrap();

        match to {
            Contact::Email(address) => {
                let template = composite.templates.email();

                let channel = &self.channels.email;

                if channel.is_none() {
                    return Err(Error::MissingChannel(ChannelType::Email));
                }

                let channel = channel.as_ref().unwrap();

                if template.is_none() {
                    return Err(Error::MissingTemplate {
                        context: "An email contact was provided, but the notification doesn't have an email template registered.",
                        notification_id: M::id().to_string(),
                        channel_type: ChannelType::Email,
                    });
                }

                let template = template.unwrap();
                let builder = template.render(&self.templates, &notification)?;

                channel.send_to(address, builder).await?;
            }
        }

        Ok(())
    }

    // /// Register a new provider P
    // pub fn register_provider<P: Provider>(&mut self, provider: P) {
    //     self.providers.register(provider)
    // }

    // /// Sends a notification to the channels associated with N for the
    // /// registered templates
    // pub async fn send<NT: Notification<Id = N>>(
    //     &self,
    //     to: &Contact,
    //     notification: N,
    // ) -> Result<usize, Error> {
    //     let templates = self
    //         .notifications
    //         .get_templates(&NT::id())
    //         .ok_or_else(|| Error::UnknownNotification(format!("{}",
    // &NT::id())))?;

    //     // iterate over the notification's templates and render each one
    //     let message_contents = templates
    //         .filter_map(|(channel_type, template)| {
    //             let provider = self.providers.get_provider(channel_type)?;
    //             Some((provider, template))
    //         })
    //         .map(|(provider, template)| {
    //             let contents = template
    //                 .render(&self.templates, &notification)
    //                 .map_err(Error::Template)?;
    //             Ok((provider, contents)) as Result<_, Error>
    //         })
    //         .collect::<Result<Vec<(&provider::DynProvider, RenderedTemplate)>,
    // Error>>()?;

    //     // iterate over the rendered templates/message contents, producing a
    // message for     // each one
    //     let messages = message_contents
    //         .into_iter()
    //         .filter(|(provider, contents)| provider.can_create_message(to,
    // contents))         .map(|(provider, contents)| {
    //             let message = provider
    //                 .create_message(to, contents)
    //                 .map_err(Error::Message)?;
    //             Ok((provider, message))
    //         })
    //         .collect::<Result<Vec<(&provider::DynProvider, message::Message)>,
    // Error>>()?;

    //     // send all the messages
    //     let out: Result<Vec<()>, Error> = stream::iter(messages.into_iter())
    //         .then(|(provider, message): (&DynProvider, Message)| async {
    //             provider.send(message).await.map_err(Error::Send)?;
    //             Ok(())
    //         })
    //         .try_collect()
    //         .await;

    //     let messages_sent = out?.len();

    //     Ok(messages_sent)
    // }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use serde::{Deserialize, Serialize};

    use crate::{email::EmailTemplate, notification::Notification, template::Markup, Notify};

    #[derive(Serialize, Deserialize)]
    pub struct NewAccountNotification {
        name: String,
        activation_url: String,
    }

    impl Notification for NewAccountNotification {
        type Id = &'static str;

        fn id() -> Self::Id {
            "new_account_notification"
        }
    }

    #[test]
    pub fn test_register_notification() {
        let email_template = EmailTemplate {
            html: Markup::MJML(indoc! {r#"
                <mjml>
                    <mj-body>
                        <mj-section>
                            <mj-column>
                                <mj-text>Hi, please verify your account by
        clicking the following link:</mj-text>
        <mj-text><a href="{{ activation_url }}">{{ activation_url
        }}</a></mj-text>                     </mj-column>
                        </mj-section>
                    </mj-body>
                </mjml>
            "#}),
            subject: "Example New Account Notification",
            text: Some(
                "Hi, please verify your account by clicking the
        following link: {{ activation_url }}",
            ),
        };

        let mut notify = Notify::default();
        let result = notify.register_template(NewAccountNotification::id(), email_template);

        assert!(result.is_ok());
    }
}
