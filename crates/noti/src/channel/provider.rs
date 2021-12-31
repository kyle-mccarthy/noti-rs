use async_trait;

use super::Error;

#[async_trait::async_trait]
pub trait Provider: Sync + Send + 'static {
    type Message;

    fn id(&self) -> &'static str;

    async fn send(&self, message: Self::Message) -> Result<(), Error>;
}
