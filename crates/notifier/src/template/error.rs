use crate::channel::ChannelType;

use super::{markup::MarkupType, TemplateId};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to render the template")]
    Render(#[source] liquid::Error),

    #[error("Failed to parse the template")]
    Parse(#[source] liquid::Error),

    #[error("Failed to parse the markup")]
    Markup {
        source: anyhow::Error,
        ty: MarkupType,
    },

    #[error("Failed to create a render context from the data")]
    InvalidData(#[source] liquid::Error),

    #[error("A template with this ID doesn't exist engine")]
    UnknownTemplate(TemplateId),

    #[error("A template hasn't been registered for this channel and notification")]
    NotFound {
        channel_type: ChannelType,
        notification_id: String,
    },
}
