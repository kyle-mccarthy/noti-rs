use super::{Address, Email, EmailBuilder};
use crate::{
    channel::{ChannelType, Error},
    id::Id,
    Notifier, Provider, RegisterChannel,
};

pub struct EmailChannel {
    pub(crate) default_sender: Option<Address>,
    pub(crate) provider: Box<dyn Provider<Message = Email>>,
}

impl EmailChannel {
    /// Create an email channel with the given provider
    pub fn new<T: Provider<Message = Email>>(provider: T) -> Self {
        Self {
            default_sender: None,
            provider: Box::new(provider),
        }
    }

    /// Set the default sender to use when sending emails. This allows for
    /// omitting the sender when building the email.
    pub fn set_default_sender(&mut self, sender: Address) {
        self.default_sender = Some(sender);
    }

    /// Send an email to the address using the builder.
    ///
    /// The builder should already contain the subject and HTML content of the
    /// email. If the channel's provider doesn't have a default email
    /// address set, the builder must also contain the sender/from address.
    pub async fn send_to(&self, to: Address, mut builder: EmailBuilder) -> Result<(), Error> {
        if builder.from.is_none() {
            if let Some(default_sender) = &self.default_sender {
                builder = builder.from_address(default_sender.clone());
            } else {
                return Err(Error::MissingField {
                    name: "from",
                    channel_type: ChannelType::Email,
                    context: Some(
                        "Missing from field and no default sender was set on the channel.",
                    ),
                });
            }
        }

        builder = builder.to_address(to);

        let message = builder.build()?;

        self.provider.send(message).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Notifier for EmailChannel {
    type Contact = Address;
    type MessageBuilder = EmailBuilder;

    async fn notify(&self, to: Self::Contact, builder: EmailBuilder) -> Result<(), crate::Error> {
        Ok(self.send_to(to, builder).await?)
    }
}

impl RegisterChannel for EmailChannel {
    fn register<N: Id>(self, instance: &mut crate::Noti<N>) {
        instance.channels.email = Some(self)
    }
}
