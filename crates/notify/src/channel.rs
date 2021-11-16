use async_trait::async_trait;
use lettre::address::AddressError;
use uuid::Uuid;

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

pub struct MessageId(Uuid);

pub enum Message {
    Email(email::Email),
}

impl Message {
    pub fn as_email(&self) -> Option<&email::Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn into_email(self) -> Option<email::Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn channel(&self) -> &'static ChannelType {
        match self {
            Self::Email(_) => &ChannelType::Email,
        }
    }
}

#[async_trait]
pub trait Channel {
    fn channel_type(&self) -> &'static ChannelType;

    // async fn before_send(&self, message: Message) -> Result<Option<Message>, Error> {
    //     Ok(Some(message))
    // }

    async fn send(&self, message: Message) -> Result<(), Error>;
}
