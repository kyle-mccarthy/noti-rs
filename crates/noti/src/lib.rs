use channel::{ChannelType, Error as ChannelError};
use contact::Contact;
use email::{EmailBuilder, EmailChannel};
use sms::{channel::SmsChannel, SmsBuilder};
use template::RenderTemplate;
use tracing::{debug, warn};

pub mod channel;
pub mod contact;
pub mod email;
pub mod id;
pub mod notification;
pub mod sms;
pub mod template;

pub use id::Id;
pub use notification::Notification;

pub trait RegisterTemplate {
    fn register<N: Id>(self, notification_id: N, notifications: &mut Noti<N>) -> Result<(), Error>;
}

pub trait RegisterChannel {
    fn register<N: Id>(self, instance: &mut Noti<N>);
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
    sms: Option<SmsChannel>,
}

#[derive(Default)]
pub struct Noti<'a, N: Id> {
    templates: template::Engine<'a>,
    notifications: notification::Store<N>,
    channels: Channels,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Template error: {0}")]
    Template(#[from] template::Error),

    #[error("The channel has not been registered. (channel type = {0})")]
    UnknownChannel(ChannelType),

    #[error("The notification has not been registered. (id = {0})")]
    UnknownNotification(String),

    #[error("Failed to create the message: {0:?}")]
    Channel(#[from] channel::Error),
}

impl<'a, N: Id> Noti<'a, N> {
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

    fn get_notification_composite(&self, id: N) -> Result<&notification::Composite, Error> {
        let composite = self.notifications.get(id);

        if composite.is_none() {
            let id = id.to_string();
            warn!(
                notification_id = id.as_ref() as &str,
                "attempting to send a notification that hasn't been registered"
            );
            return Err(Error::UnknownNotification(id));
        }

        let composite = composite.unwrap();

        Ok(composite)
    }

    fn get_email_channel(&self) -> Result<&EmailChannel, Error> {
        let channel = &self.channels.email;

        if channel.is_none() {
            warn!("attempting to send an email notification but no email provider is registered");
            return Err(Error::UnknownChannel(ChannelType::Email));
        }

        let channel = channel.as_ref().unwrap();

        Ok(channel)
    }

    fn get_sms_channel(&self) -> Result<&SmsChannel, Error> {
        let channel = &self.channels.sms;

        if channel.is_none() {
            warn!("attempting to send an email notification but no email provider is registered");
            return Err(Error::UnknownChannel(ChannelType::Email));
        }

        let channel = channel.as_ref().unwrap();

        Ok(channel)
    }

    fn create_email_notification<M: Notification<Id = N>>(
        &self,
        notification: &M,
    ) -> Result<EmailBuilder, Error> {
        let composite = self.get_notification_composite(M::id())?;

        let template = composite.templates.email();

        if template.is_none() {
            return Err(ChannelError::MissingTemplate {
                context: Some(
                    "An email contact was provided, but the notification doesn't\
                have an email template registered.",
                ),
                notification_id: M::id().to_string(),
                channel_type: ChannelType::Email,
            }.into());
        }

        let template = template.unwrap();
        let builder = template.render(&self.templates, &notification)?;

        Ok(builder)
    }

    fn create_sms_notification<M: Notification<Id = N>>(
        &self,
        notification: &M,
    ) -> Result<SmsBuilder, Error> {
        let composite = self.get_notification_composite(M::id())?;

        let template = composite.templates.sms();

        if template.is_none() {
            return Err(ChannelError::MissingTemplate {
                context: Some(
                    "An email contact was provided, but the notification doesn't\
                have an email template registered.",
                ),
                notification_id: M::id().to_string(),
                channel_type: ChannelType::Email,
            }.into());
        }

        let template = template.unwrap();
        let builder = template.render(&self.templates, &notification)?;

        Ok(builder)
    }

    pub async fn send<M: Notification<Id = N>>(
        &self,
        to: Contact,
        notification: M,
    ) -> Result<(), Error> {
        match to {
            Contact::Email(address) => {
                let channel = self.get_email_channel()?;

                let builder = self.create_email_notification(&notification)?;

                debug!("sending an email notification");

                channel.send_to(address, builder).await?;
            }
            Contact::PhoneNumber(phone_number) => {
                let channel = self.get_sms_channel()?;
                let builder = self.create_sms_notification(&notification)?;

                debug!("sending an sms notification");

                channel.send_to(phone_number, builder).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use serde::{Deserialize, Serialize};

    use crate::{email::EmailTemplate, notification::Notification, template::Markup, Noti};

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

        let mut notify = Noti::default();
        let result = notify.register_template(NewAccountNotification::id(), email_template);

        assert!(result.is_ok());
    }
}
