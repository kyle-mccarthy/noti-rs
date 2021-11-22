use async_trait::async_trait;
use lettre::address::AddressError;

use crate::message::Message;

pub mod email;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build the message: {0:?}")]
    Message(anyhow::Error),

    #[error("Failed to parse the address: {0:?}")]
    Address(#[from] AddressError),

    #[error("Email channel encountered")]
    Email(#[from] email::Error),

    #[error(
        "Channel received message of incorrect type: (expected {expected:?}, found {found:?})"
    )]
    InvalidMessageChannel {
        found: &'static ChannelType,
        expected: &'static ChannelType,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelType {
    Email,
}

#[async_trait]
pub trait Channel {
    fn channel_type(&self) -> &'static ChannelType;

    async fn send(&self, message: Message) -> Result<(), Error>;
}
