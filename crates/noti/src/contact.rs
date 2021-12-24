use async_trait::async_trait;

use crate::{email::Address, id::Id};

pub mod error;
pub use error::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Contact {
    Email(crate::email::Address),
}

#[async_trait]
pub trait ContactRepository: Sync {
    type Id: Id;

    async fn name(&self, id: Self::Id) -> Result<String, Error>;

    async fn email(&self, id: Self::Id) -> Result<String, Error>;

    // async fn phone_number(&self, _id: Self::Id) -> Option<String> {
    //     None
    // }

    // async fn device_id(&self, _id: Self::Id) -> Option<String> {
    //     None
    // }

    // async fn via(
    //     &self,
    //     id: Self::Id,
    //     notification_id: &str,
    // ) -> Result<Option<Vec<ChannelType>>, Error>;
}

impl From<Address> for Contact {
    fn from(address: Address) -> Self {
        Self::Email(address)
    }
}
