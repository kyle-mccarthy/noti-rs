use async_trait::async_trait;

use crate::{contact::Contact, notification::Notification, Error, Notify};

#[async_trait(?Send)]
pub trait Dispatch<T> {
    async fn dispatch<N: Notification>(&self, to: &T, notification: N) -> Result<(), Error>;
}

// #[async_trait(?Send)]
// impl Dispatch<Contact> for Notify {
//     async fn dispatch<N: Notification>(&self, to: &Contact, notification: N) -> Result<(), Error> {
//         self.send(to, notification).await?;
//         Ok(())
//     }
// }
