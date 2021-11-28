use std::fmt::Display;

use crate::{contact::Contact, template::RenderedTemplate};

pub mod email;
pub use email::EmailChannel;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Channel failed to create the message")]
    Message(String),
}

pub trait Channel: sealed::Sealed {
    type Message;
    type Contents;

    fn channel_type() -> ChannelType;

    fn can_create_message(contact: &Contact, template: &RenderedTemplate) -> bool;

    fn create_message(contact: Contact, template: Self::Contents) -> Result<Self::Message, Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
