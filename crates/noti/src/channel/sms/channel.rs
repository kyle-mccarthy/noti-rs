use super::{PhoneNumber, Sms, SmsBuilder};
use crate::{
    channel::{ChannelType, Error},
    Id, Notifier, Provider, RegisterChannel,
};

pub struct SmsChannel {
    pub(crate) default_sender: Option<PhoneNumber>,
    pub(crate) provider: Box<dyn Provider<Message = Sms>>,
}

impl SmsChannel {
    pub fn new<T: Provider<Message = Sms>>(provider: T) -> Self {
        Self {
            default_sender: None,
            provider: Box::new(provider),
        }
    }

    pub fn set_default_sender(&mut self, sender: PhoneNumber) {
        self.default_sender = Some(sender);
    }

    pub async fn send_to(&self, to: PhoneNumber, mut builder: SmsBuilder) -> Result<(), Error> {
        if builder.from.is_none() {
            if let Some(default_sender) = &self.default_sender {
                builder = builder.from(default_sender.clone());
            } else {
                return Err(Error::MissingField {
                    name: "from",
                    channel_type: ChannelType::Sms,
                    context: None,
                });
            }
        }

        builder = builder.to(to);

        let message = builder.build()?;

        self.provider.send(message).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl Notifier for SmsChannel {
    type Contact = PhoneNumber;
    type MessageBuilder = SmsBuilder;

    async fn notify(&self, to: Self::Contact, builder: SmsBuilder) -> Result<(), crate::Error> {
        Ok(self.send_to(to, builder).await?)
    }
}

impl RegisterChannel for SmsChannel {
    fn register<N: Id>(self, instance: &mut crate::Noti<N>) {
        instance.channels.sms = Some(self)
    }
}
