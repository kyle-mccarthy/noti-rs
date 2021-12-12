use std::{any::TypeId, collections::HashMap};

use fixed_map::{map::Iter, Map};

use super::Notification;
use crate::{channel::ChannelType, template::RegisteredTemplate};

#[derive(Default)]
pub struct Manager {
    notifications: HashMap<TypeId, NotificationConfig>,
}

impl Manager {
    /// Set the template to use for the notification on the given channel
    pub fn set_template<N: Notification>(
        &mut self,
        channel: ChannelType,
        template: RegisteredTemplate,
    ) -> Option<RegisteredTemplate> {
        let config = self.notifications.entry(TypeId::of::<N>()).or_default();
        config.templates.insert(channel, template)
    }

    /// Get the template to use for the notification's channel
    pub fn get_template<N: Notification>(
        &self,
        notification: &N,
        channel: ChannelType,
    ) -> Option<&RegisteredTemplate> {
        self.notifications
            .get(&notification.type_id())
            .map(|item| item.templates.get(channel))
            .flatten()
    }

    /// Get an iterator over the templates for a given notification
    pub fn get_templates<N: Notification>(
        &self,
        notification: &N,
    ) -> Option<Iter<ChannelType, RegisteredTemplate>> {
        self.notifications
            .get(&notification.type_id())
            .map(|item| item.templates.iter())
    }
}

#[derive(Default)]
struct NotificationConfig {
    templates: Map<ChannelType, RegisteredTemplate>,
}
