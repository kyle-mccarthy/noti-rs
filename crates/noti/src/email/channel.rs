use super::{Address, EmailBuilder, EmailProvider};
use crate::{
    channel::{ChannelType, Error},
    id::Id,
    Notifier, RegisterChannel,
};

pub struct EmailChannel {
    pub(crate) default_sender: Option<Address>,
    pub(crate) provider: Box<dyn EmailProvider>,
}

impl EmailChannel {
    pub fn new<T: EmailProvider + 'static>(provider: T) -> Self {
        Self {
            default_sender: None,
            provider: Box::new(provider),
        }
    }

    pub fn set_default_sender(&mut self, sender: Address) {
        self.default_sender = Some(sender);
    }

    // pub async fn create_contact<C: Id>(
    //     &self,
    //     contact_id: C,
    //     repository: &impl ContactRepository<Id = C>,
    // ) -> Result<Address, crate::contact::Error> {
    //     let email = repository.email(contact_id).await?;
    //     let name = repository.name(contact_id).await.ok();

    //     Ok(Address::new(email, name))
    // }

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
