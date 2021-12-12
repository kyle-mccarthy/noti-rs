use std::fmt::Display;

use fixed_map::Key;

use crate::{contact::Contact, message::Message, provider::Error, template::RenderedTemplate};

pub mod email;
pub use email::EmailChannel;

pub trait Channel: sealed::Sealed {
    type Message;
    type Contents;

    /// The channel's type
    fn channel_type() -> ChannelType;

    /// Returns true if the channel can create a message for the given contact
    /// and message contents
    fn can_create_message(contact: &Contact, contents: &RenderedTemplate) -> bool;

    /// Create a message for the given contact and message contents.
    fn create_message(contact: &Contact, contents: Self::Contents) -> Result<Self::Message, Error>;

    /// Attempts to downcast/convert the rendered template's contents into the
    /// type expected by the Channel
    fn downcast_contents(contents: RenderedTemplate) -> Option<Self::Contents>;

    /// Upcasts/converts the channel's message type into a generalized Message
    fn upcast_message(message: Self::Message) -> Message;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Key)]
pub enum ChannelType {
    Email,
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => f.write_str("Email"),
        }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::EmailChannel {}
}
