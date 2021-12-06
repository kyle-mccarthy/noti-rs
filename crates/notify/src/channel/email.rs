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

    /// Returns the channel type for EmailChannel
    fn channel_type() -> ChannelType {
        ChannelType::Email
    }

    /// Create an Email from the contents
    fn create_message(contact: &Contact, contents: Self::Contents) -> Result<Self::Message, Error> {
        let to = contact
            .email()
            .ok_or_else(|| Error::Contact("Contact missing required email field.".to_string()))?;

        Ok(Email::new(to.to_string(), contents))
    }

    /// Checks if an Email can be created based on the Contact and contents of
    /// the rendered Template
    fn can_create_message(contact: &Contact, template: &RenderedTemplate) -> bool {
        contact.has_email() && template.is_email()
    }

    /// Attempts to downcast the contents of the rendered template into the
    /// contents expected
    fn downcast_contents(contents: RenderedTemplate) -> Option<Self::Contents> {
        contents.into_email()
    }

    /// Upcasts/wraps the Email into a Message
    fn upcast_message(message: Self::Message) -> crate::message::Message {
        Message::Email(message)
    }
}
