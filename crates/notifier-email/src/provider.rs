use notifier::Error;

use crate::EmailMessage;

#[cfg(feature = "smtp")]
pub mod smtp;

#[async_trait::async_trait]
pub trait Provider: Sync + Send + 'static {
    fn id(&self) -> &'static str;

    async fn send(&self, email: EmailMessage) -> Result<(), Error>;
}
