use lettre::address::AddressError;

mod email;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build the message: {0:?}")]
    Message(anyhow::Error),

    #[error("Failed to parse the address: {0:?}")]
    Address(#[from] AddressError),
}

pub struct Channel {}
