use std::fmt::Display;

mod error;
mod provider;

pub mod email;
pub mod push;
pub mod sms;

pub use error::Error;
pub use provider::Provider;

use crate::{Id, Notification};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelType {
    Email,
    Sms,
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => f.write_str("Email"),
            Self::Sms => f.write_str("SMS"),
        }
    }
}

pub trait Channel {
    type Contact;
    type Message;
    type Template;

    fn register_template<N: Id>(
        &mut self,
        notification_id: &N,
        template: Self::Template,
    ) -> Result<(), Error>;

    fn create_message<N: Notification>(
        &self,
        notification: &N,
        contact: &Self::Contact,
    ) -> Result<Self::Message, Error>;

    fn send(&self, message: Self::Message) -> Result<(), Error>;
}

pub trait TemplateRepository {}
