use std::any::Any;

use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Id, Notification};

#[async_trait::async_trait]
pub trait Channel<I: Id>: Any + Sync + Send {
    type Contact;
    type Message: Serialize + DeserializeOwned;

    /// Create a message that has the contact as the recipient
    fn create_message<N: Notification<Id = I>>(
        &self,
        notification: &N,
        contact: Self::Contact,
    ) -> Result<Self::Message, Error>;

    /// Send a message using the channel's provider
    async fn send(&self, message: Self::Message) -> Result<(), Error>;
}

pub trait RegisterNotification<'a, I: Id> {
    type Template;

    /// Register the template with the channel
    fn register_notification<N: Notification<Id = I>>(
        &mut self,
        template: Self::Template,
    ) -> Result<(), Error>;
}
