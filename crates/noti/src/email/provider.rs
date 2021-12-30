use async_trait::async_trait;

use super::{Address, Email};
use crate::channel::Error;

pub mod smtp;
pub use smtp::SmtpProvider;

#[async_trait]
pub trait EmailProvider: Sync + Send + 'static {
    fn id(&self) -> &'static str;

    async fn send(&self, email: Email) -> Result<(), Error>;
}
