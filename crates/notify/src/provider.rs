use std::{any::Any, collections::HashMap};

use async_trait::async_trait;

use crate::{
    channel::{Channel, ChannelType},
    message::Message,
};

pub mod smtp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unexpected channel type (expected {expected}, found {found})")]
    ChannelType {
        expected: ChannelType,
        found: ChannelType,
    },
}

#[async_trait]
pub trait Provider: Any {
    type Channel: Channel;

    async fn send(&self, message: <Self::Channel as Channel>::Message) -> Result<(), Error>;

    fn channel_type(&self) -> ChannelType {
        <Self::Channel as Channel>::channel_type()
    }

    fn id(&self) -> &str;

    fn into_dyn_provider(self) -> DynProvider;
}

#[async_trait]
pub trait EmailProvider {
    async fn send(&self, message: crate::message::email::Email) -> Result<(), Error>;

    fn channel_type(&self) -> ChannelType {
        ChannelType::Email
    }

    fn id(&self) -> &str;
}

pub enum DynProvider {
    Email(Box<dyn Provider<Channel = crate::channel::EmailChannel>>),
}

impl DynProvider {
    pub async fn send(&self, message: Message) -> Result<(), Error> {
        match self {
            Self::Email(provider) => {
                if !message.is_email() {
                    return Err(Error::ChannelType {
                        expected: ChannelType::Email,
                        found: message.channel_type(),
                    });
                }
                // safety: we've verified the message's channel type + can safely unwrap
                let email = message.into_email().unwrap();
                provider.send(email).await
            }
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Email(provider) => provider.id(),
        }
    }

    pub fn channel_type(&self) -> ChannelType {
        match self {
            Self::Email(provider) => provider.channel_type(),
        }
    }
}

#[derive(Default)]
pub struct Manager {
    providers: HashMap<ChannelType, DynProvider>,
}

impl Manager {
    pub fn register<P: Provider + 'static>(&mut self, provider: P) {
        let channel_type = provider.channel_type();
        let provider: DynProvider = provider.into_dyn_provider();
        self.providers.insert(channel_type, provider);
    }

    pub fn get_provider(&self, channel_type: ChannelType) -> Option<&DynProvider> {
        self.providers.get(&channel_type)
    }
}
