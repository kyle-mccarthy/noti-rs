use notifier::{
    channel::RegisterNotification, template::TemplateService, Channel, Error, Id, Notification,
};

pub mod contact;
pub mod message;
pub mod provider;
pub mod template;

pub use contact::EmailAddress;
pub use message::{EmailContents, EmailMessage};
use provider::Provider;
pub use template::EmailTemplate;
use template::RegisteredEmailTemplate;

pub struct Options {
    /// The default sender to use for the email
    default_sender: EmailAddress,
    /// An optional reply_to email address
    reply_to: Option<EmailAddress>,
}

pub struct EmailChannel<N: Id> {
    templates: TemplateService<N, RegisteredEmailTemplate>,
    provider: Box<dyn Provider>,
    options: Options,
}

impl<I: Id> EmailChannel<I> {
    pub fn new(provider: impl Provider, options: Options) -> Self {
        Self {
            templates: TemplateService::new(),
            provider: Box::new(provider),
            options,
        }
    }

    pub fn create_message<N: Notification<Id = I>>(
        &self,
        notification: &N,
        contact: EmailAddress,
    ) -> Result<EmailMessage, Error> {
        let contents = self.templates.render(notification)?;

        let mut message = EmailMessage::new(contact, self.options.default_sender.clone(), contents);

        message.set_reply_to(self.options.reply_to.clone());

        Ok(message)
    }

    pub async fn send(&self, message: EmailMessage) -> Result<(), Error> {
        self.provider.send(message).await
    }
}

#[async_trait::async_trait]
impl<I: Id + 'static> Channel<I> for EmailChannel<I> {
    type Contact = EmailAddress;
    type Message = EmailMessage;

    /// Create a message that has the contact as the recipient
    fn create_message<N: Notification<Id = I>>(
        &self,
        notification: &N,
        contact: Self::Contact,
    ) -> Result<Self::Message, Error> {
        self.create_message(notification, contact)
    }

    /// Send a message using the channel's provider
    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        self.send(message).await
    }
}

impl<'a, I: Id> RegisterNotification<'a, I> for EmailChannel<I> {
    type Template = EmailTemplate<'a>;

    fn register_notification<N: Notification<Id = I>>(
        &mut self,
        source: Self::Template,
    ) -> Result<(), Error> {
        self.templates.register::<N, Self::Template>(source)?;
        Ok(())
    }
}
