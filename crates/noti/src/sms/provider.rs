use async_trait::async_trait;

use super::Sms;
use crate::channel::Error;

#[cfg(feature = "twilio")]
pub mod twilio;

#[async_trait]
pub trait SmsProvider: Sync + Send + 'static {
    fn id(&self) -> &'static str;

    async fn send(&self, message: Sms) -> Result<(), Error>;
}
