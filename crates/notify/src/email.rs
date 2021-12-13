use serde::{Deserialize, Serialize};

pub mod message;
pub mod provider;
pub mod channel;

pub use message::{Address, Email, EmailBuilder};
pub use provider::EmailProvider;
pub use channel::EmailChannel;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Email missing required field")]
    MissingField(&'static str),

    #[error("Email address invalid: {0}")]
    Address(String),

    #[error("Failed to create email: {0}")]
    Message(String),

    #[error("{0}")]
    Transport(String),

    #[error("{0}")]
    Send(String),
}
