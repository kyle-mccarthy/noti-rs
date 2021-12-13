use async_trait::async_trait;

use super::{Address, Email, Error};

pub mod smtp;
pub use smtp::SmtpProvider;

#[async_trait]
pub trait EmailProvider {
    fn id(&self) -> &str;

    async fn send(&self, email: Email) -> Result<(), Error>;
}
