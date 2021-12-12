use async_trait::async_trait;

use crate::channel::ChannelType;

#[async_trait]
pub trait ContactRepository {
    type Id: Sync + Send;

    async fn name(&self, _id: Self::Id) -> Option<String> {
        None
    }

    async fn email(&self, _id: Self::Id) -> Option<String> {
        None
    }

    async fn phone_number(&self, _id: Self::Id) -> Option<String> {
        None
    }

    async fn device_id(&self, _id: Self::Id) -> Option<String> {
        None
    }

    async fn should_send(
        &self,
        _id: Self::Id,
        _channel: ChannelType,
        _notification_id: &str,
    ) -> bool {
        true
    }
}

#[async_trait]
pub trait PreferenceRepository {
    type Contacts: ContactRepository;

    async fn should_send(&self, contact_id: <Self::Contacts as ContactRepository>::Id);
}
