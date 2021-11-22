use serde::{Deserialize, Serialize};

use super::Message;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Email {
    /// The recipient of the email.
    pub to: String,
    /// The sender of the email. This is optional, but the email provider may
    /// require it if can't accept default senders.
    pub from: Option<String>,
    /// Subject to use for the email.
    pub subject: String,
    /// HTML version of the email.
    pub html: String,
    /// Optional plain text version of the email.
    pub text: Option<String>,
}

impl Email {
    pub fn into_message(self) -> Message {
        Message::Email(self)
    }
}
