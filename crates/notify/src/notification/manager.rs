use std::collections::HashMap;

use fixed_map::{map::Iter, Map};

use crate::{channel::ChannelType, id::Id, template::RegisteredTemplate};

#[derive(Default)]
pub struct Manager<T: Id> {
    notifications: HashMap<T, NotificationConfig>,
}

impl<T: Id> Manager<T> {
    /// Set the template to use for the notification on the given channel
    pub fn set_template(
        &mut self,
        notification_id: T,
        channel: ChannelType,
        template: RegisteredTemplate,
    ) -> Option<RegisteredTemplate> {
        let config = self.notifications.entry(notification_id).or_default();
        config.templates.insert(channel, template)
    }

    /// Get the template to use for the notification's channel
    pub fn get_template(
        &self,
        notification_id: &T,
        channel: ChannelType,
    ) -> Option<&RegisteredTemplate> {
        self.notifications
            .get(notification_id)
            .map(|item| item.templates.get(channel))
            .flatten()
    }

    /// Get an iterator over the templates for a given notification
    pub fn get_templates(
        &self,
        notification_id: &T,
    ) -> Option<Iter<ChannelType, RegisteredTemplate>> {
        self.notifications
            .get(notification_id)
            .map(|item| item.templates.iter())
    }
}

#[derive(Default)]
struct NotificationConfig {
    templates: Map<ChannelType, RegisteredTemplate>,
}
