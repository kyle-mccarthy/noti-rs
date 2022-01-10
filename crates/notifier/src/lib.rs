pub mod channel;
pub mod notification;
pub mod template;

pub use channel::Channel;
pub use notification::{Id, Notification};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TemplateError")]
    Template(#[from] template::Error),

    // Contact {
    //     source: anyhow::Error,
    //     channel: &'static str,

    // }

    #[error("The channels's provider encountered an error")]
    Provider {
        source: anyhow::Error,
        channel: &'static str,
        provider_id: &'static str,
        context: Option<&'static str>,
    }
}
