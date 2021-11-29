use super::{Channel, ChannelType, Error};
use crate::{
    contact::Contact,
    message::{
        email::{Email, EmailContents},
        Message,
    },
    template::RenderedTemplate,
};

pub struct EmailChannel;

impl Channel for EmailChannel {
    type Contents = EmailContents;
    type Message = Email;

    fn channel_type() -> ChannelType {
        ChannelType::Email
    }

    fn create_message(contact: &Contact, contents: Self::Contents) -> Result<Self::Message, Error> {
        let to = contact
            .email()
            .ok_or_else(|| Error::Contact("Contact missing required email field.".to_string()))?;

        Ok(Email::new(to.to_string(), contents))
    }

    fn can_create_message(contact: &Contact, template: &RenderedTemplate) -> bool {
        contact.is_email() && template.is_email()
    }

    fn downcast_contents(contents: RenderedTemplate) -> Option<Self::Contents> {
        contents.into_email()
    }

    fn upcast_message(message: Self::Message) -> crate::message::Message {
        Message::Email(message)
    }
}
