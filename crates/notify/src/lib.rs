pub use notify_macros::EmailNotification;
use template::TemplateManager;

pub mod email;
pub mod template;
pub mod channel;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0:?}")]
    Template(#[from] template::Error),

    #[error("{0:?}")]
    Channel(#[from] channel::Error),
}

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub enum ChannelType {
    Email,
}

#[derive(Default)]
pub struct Notify<'a> {
    templates: TemplateManager<'a>,
}

impl<'a> Notify<'a> {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
