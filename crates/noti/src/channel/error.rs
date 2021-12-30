use super::ChannelType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Builder failed to create the message. A required field on the contact is missing. (name = {name})")]
    MissingField {
        name: &'static str,
        channel_type: ChannelType,
        context: Option<&'static str>,
    },

    #[error("The channel does not have a template registered for this notification.")]
    MissingTemplate {
        notification_id: String,
        channel_type: ChannelType,
        context: Option<&'static str>,
    },

    #[error("The contact isn't valid.")]
    InvalidContact {
        source: anyhow::Error,
        context: Option<&'static str>,
        channel_type: ChannelType,
    },

    #[error("The channel failed to send the message.")]
    Send {
        source: anyhow::Error,
        context: Option<&'static str>,
        channel_type: ChannelType,
        provider_id: &'static str,
    },
}
