use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use notifier::{provider::Error, Provider};

use crate::EmailMessage;

#[derive(Default)]
pub struct TestProvider(pub Arc<Mutex<Vec<EmailMessage>>>);

impl Clone for TestProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl TestProvider {
    pub async fn send(&self, message: EmailMessage) -> Result<(), Error> {
        let mut lock = self.0.lock().expect("failed to lock mutext");
        lock.push(message);
        Ok(())
    }
}

#[async_trait]
impl Provider for TestProvider {
    type Message = EmailMessage;

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        self.send(message).await
    }

    fn id(&self) -> &'static str {
        "test"
    }
}
