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
}

pub struct MessageId(Uuid);

pub enum Message {
    Email(email::Email),
}

#[async_trait]
pub trait Channel {
    #[inline]
    async fn before_send(&self, message: Message) -> Result<Option<Message>, Error> {
        Ok(Some(message))
    }

    async fn send(&self, message: Message) -> Result<MessageId, Error>;
}
