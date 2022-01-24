use std::any::Any;

use serde::{de::DeserializeOwned, Serialize};

use crate::channel::ChannelType;

pub trait Contact: Any + Send + Serialize + DeserializeOwned {}

impl<T: Any + Send + Serialize + DeserializeOwned> Contact for T {}

/// Wraps the contact as Box<dyn Any + Send> and is tagged with the channel's
/// ChannelType
pub struct DynContact {
    contact: Box<dyn Any + Send>,
    channel: ChannelType,
}

impl DynContact {
    pub fn new(contact: impl Any + Send, channel: ChannelType) -> Self {
        Self {
            contact: Box::new(contact),
            channel,
        }
    }

    pub fn channel(&self) -> ChannelType {
        self.channel
    }

    pub fn contact(&self) -> &(dyn Any + Send) {
        &self.contact
    }

    pub fn take_contact(self) -> Box<dyn Any + Send> {
        self.contact
    }
}
