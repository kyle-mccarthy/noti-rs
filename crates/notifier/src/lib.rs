pub mod channel;
pub mod contact;
pub mod message;
pub mod notification;
pub mod provider;
pub mod template;

use std::any::{Any, TypeId};

use channel::registry::ChannelRegistry;
pub use channel::Channel;
use contact::{Contact, DynContact};
pub use notification::{Id, Notification};
pub use provider::{Error as ProviderError, Provider};
pub use template::TemplateError;
use template::{engine::RenderContext, TemplateService};

#[cfg(test)]
pub(crate) mod test_utils;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TemplateError: {0:?}")]
    Template(#[from] TemplateError),

    #[error("ProviderError: {0:?}")]
    Provider(#[from] ProviderError),

    #[error("Channel could not be found: {0}")]
    UnknownChannel(&'static str),

    #[error("Failed to downcast")]
    Downcast {
        found: TypeId,
        expected: TypeId,
        context: Option<&'static str>,
    },
}

#[derive(Default)]
pub struct Notifier<I: Id> {
    channels: ChannelRegistry<I>,
    templates: TemplateService<I>,
}

impl<I: Id + Default> Notifier<I> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<I: Id> Notifier<I> {
    /// Add the channel to the notifier's registry.
    pub fn register_channel<C: Channel<I>>(&mut self, channel: C) {
        self.channels.register(channel)
    }

    /// Register a template for the notification.
    pub fn register_notification<N: Notification<Id = I>, T: Any>(
        &mut self,
        template: T,
    ) -> Result<(), Error> {
        let channel = self
            .channels
            .find_by_template::<T>()
            .ok_or(Error::UnknownChannel(
                "A channel for this template type has not yet been registered.",
            ))?;

        channel.register_dyn_template(N::id(), Box::new(template), &mut self.templates)?;

        Ok(())
    }

    /// Send the message to a specific channel's contact.
    pub async fn send_message_to_contact<N: Notification<Id = I>, C: Contact>(
        &self,
        notification: N,
        contact: C,
    ) -> Result<(), Error> {
        let channel = self
            .channels
            .find_by_contact::<C>()
            .ok_or(Error::UnknownChannel(
                "A channel for this contact type has not yet been registered.",
            ))?;

        let notification_id = N::id();
        let context = RenderContext::with_data(&notification)?;

        let dyn_contents =
            channel.render_dyn_template(notification_id, &context, &self.templates)?;

        let dyn_contact = DynContact::new(contact, channel.get_channel_type());

        let dyn_message = channel.create_dyn_message(dyn_contact, dyn_contents)?;

        channel.send_dyn_message(dyn_message).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{test_utils::*, *};

    #[test]
    fn test_register_notification() {
        let mut notifier = Notifier::<&'static str>::default();

        let channel = TestChannel::default();

        notifier.register_channel(channel);

        let template = TestTemplate("message = {{message}}");

        let result = notifier.register_notification::<TestNotification, TestTemplate>(template);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_notification() {
        let mut notifier = Notifier::<&'static str>::default();

        let channel = TestChannel::default();
        let template = TestTemplate("message = {{message}}");

        notifier.register_channel(channel.clone());

        notifier
            .register_notification::<TestNotification, TestTemplate>(template)
            .unwrap();

        let notification = TestNotification::new(1, "first notification".to_string());
        let contact = TestContact("Destination (1)".to_string());

        let result = notifier
            .send_message_to_contact(notification, contact)
            .await;

        assert!(result.is_ok());

        let messages = channel.messages.lock().unwrap();
        let len = messages.len();
        assert_eq!(len, 1);
    }

    #[tokio::test]
    async fn test_fails_to_register_notification_on_unknown_channel() {
        let mut notifier = Notifier::<&'static str>::default();
        let template = TestTemplate("message = {{message}}");

        let result = notifier.register_notification::<TestNotification, TestTemplate>(template);

        assert!(result.is_err());
        assert!(matches!(result, Err(Error::UnknownChannel(_))));
    }

    #[tokio::test]
    async fn test_fails_to_send_unknown_notification() {
        let mut notifier = Notifier::<&'static str>::default();

        let channel = TestChannel::default();

        notifier.register_channel(channel.clone());

        let notification = TestNotification::new(1, "first notification".to_string());
        let contact = TestContact("Destination (1)".to_string());

        let result = notifier
            .send_message_to_contact(notification, contact)
            .await;

        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(Error::Template(TemplateError::NotFound { .. }))
        ));
    }
}
