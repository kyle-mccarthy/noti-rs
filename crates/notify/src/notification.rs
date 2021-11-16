use serde::Serialize;

use crate::{channel::ChannelType, template::Template};

pub trait Notification {
    type Template: Template;
    type Data: Serialize;

    const CHANNEL_TYPE: &'static ChannelType;

    fn data(&self) -> &Self::Data;
}
