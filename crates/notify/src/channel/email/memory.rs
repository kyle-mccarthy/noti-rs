use notify_macros::EmailProvider;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use super::{Email, EmailProvider, Error};

#[derive(EmailProvider)]
pub struct InMemoryChannel {
    sender: Sender<Email>,
    receiver: Option<Receiver<Email>>,
}

impl InMemoryChannel {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = channel(capacity);
        Self {
            sender,
            receiver: Some(receiver),
        }
    }

    pub fn sender(&self) -> Sender<Email> {
        self.sender.clone()
    }

    pub fn receiver(&mut self) -> &mut Option<Receiver<Email>> {
        &mut self.receiver
    }
}

impl Default for InMemoryChannel {
    fn default() -> Self {
        let (sender, receiver) = channel(100);
        Self {
            sender,
            receiver: Some(receiver),
        }
    }
}

#[async_trait::async_trait]
impl EmailProvider for InMemoryChannel {
    async fn send(&self, message: Email) -> Result<(), Error> {
        self.sender
            .send(message)
            .map_err(|e| Error::SendFailed(e.into()))?;

        Ok(())
    }
}
