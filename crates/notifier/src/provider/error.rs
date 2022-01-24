#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ContactError")]
    Contact {
        channel_id: &'static str,
        provider_id: &'static str,
        source: anyhow::Error,
        context: Option<&'static str>,
    },

    #[error("SendError")]
    Send {
        channel_id: &'static str,
        provider_id: &'static str,
        source: anyhow::Error,
        context: Option<&'static str>,
    },

    #[error("Provider had unknown error")]
    Unknown {
        channel_id: &'static str,
        provider_id: &'static str,
        source: anyhow::Error,
        context: Option<&'static str>,
    },
}
