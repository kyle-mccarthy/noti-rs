use std::{
    any::Any,
    collections::{HashMap, HashSet},
};

use crate::{channel::ChannelType, Id};

#[derive(Default)]
pub struct TemplateRegistry<I: Id> {
    notifications: HashMap<I, HashSet<ChannelType>>,
    templates: HashMap<(I, ChannelType), Box<dyn Any>>,
}

impl<I: Id> TemplateRegistry<I> {
    pub fn new() -> Self {
        Self {
            notifications: HashMap::new(),
            templates: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        notification_id: I,
        channel_type: ChannelType,
        template: Box<dyn Any>,
    ) {
        let entry = self.notifications.entry(notification_id).or_default();
        entry.insert(channel_type);
        self.templates
            .insert((notification_id, channel_type), template);
    }

    pub fn get_template(
        &self,
        notification_id: I,
        channel_type: ChannelType,
    ) -> Option<&Box<dyn Any>> {
        self.templates.get(&(notification_id, channel_type))
    }
}
