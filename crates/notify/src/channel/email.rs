use super::{Channel, ChannelType, Error};
use crate::{
    contact::Contact,
    message::email::{Email, EmailContents},
    template::RenderedTemplate,
};

pub struct EmailChannel;

impl Channel for EmailChannel {
    type Contents = EmailContents;
    type Message = Email;

    fn channel_type() -> ChannelType {
        ChannelType::Email
    }

    fn create_message(contact: Contact, template: Self::Contents) -> Result<Self::Message, Error> {
        todo!()
    }

    fn can_create_message(contact: &Contact, template: &RenderedTemplate) -> bool {
        contact.has_email() && template.is_email()
    }
}

// pub struct EmailChannel(Arc<dyn EmailProvider>);

// pub trait EmailProvider {
//     fn send(&self, message: Email) -> Result<(), Error>;
// }

// impl Channel for EmailChannel {
//     type Message = Email;
// }
