use std::any::Any;

use erased_serde::Serialize;

use crate::channel::ChannelType;

pub trait Message: Any + Serialize + Send {}

// serialize_trait_object!(Message);

impl<T: Any + Serialize + Send> Message for T {}

// static_assertions::assert_obj_safe!(Message);

/// Wraps the message as Box<dyn Any + Send> and is tagged with the channel's
/// ChannelType
pub struct DynMessage {
    message: Box<dyn Any + Send>,
    channel: ChannelType,
}

impl DynMessage {
    pub fn new(message: impl Any + Send, channel: ChannelType) -> Self {
        Self {
            message: Box::new(message),
            channel,
        }
    }

    pub fn channel_type(&self) -> ChannelType {
        self.channel
    }

    pub fn message(&self) -> &(dyn Any + Send) {
        &self.message
    }

    pub fn take_message(self) -> Box<dyn Any + Send> {
        self.message
    }
}

pub struct DynMessageContents {
    contents: Box<dyn Any + Send>,
    channel: ChannelType,
}

impl DynMessageContents {
    pub fn new(contents: impl Any + Send, channel: ChannelType) -> Self {
        Self {
            contents: Box::new(contents),
            channel,
        }
    }

    pub fn channel_type(&self) -> ChannelType {
        self.channel
    }

    pub fn contents(&self) -> &(dyn Any + Send) {
        &self.contents
    }

    pub fn take_contents(self) -> Box<dyn Any + Send> {
        self.contents
    }
}
