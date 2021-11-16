use std::collections::HashMap;

use channel::ChannelType;
pub use notify_macros::{EmailNotification, EmailProvider};
use template::{Template, TemplateManager};

pub mod channel;
pub mod notification;
pub mod template;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Template(#[from] template::Error),

    #[error("{0:?}")]
    Channel(#[from] channel::Error),
}

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Default)]
pub struct Notify<'a> {
    templates: TemplateManager<'a>,
    channels: HashMap<&'static ChannelType, Box<dyn channel::Channel>>,
}

impl<'a> Notify<'a> {
    pub fn register_provider<C: channel::Channel + 'static>(&mut self, provider: C) {
        self.channels
            .insert(provider.channel_type(), Box::new(provider));
    }

    pub fn register_template<T: Template>(&mut self) -> Result<(), Error> {
        Ok(self.templates.register::<T>()?)
    }
}

pub trait RegisterProvider<T> {
    fn register_provider(&mut self, provider: T);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
