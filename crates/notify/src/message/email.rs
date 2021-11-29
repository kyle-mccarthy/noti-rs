use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Email {
    /// The recipient of the email.
    pub to: String,
    /// The sender of the email. This is optional, but the email provider may
    /// require it if can't accept default senders.
    pub from: Option<String>,
    /// The email's contents.
    pub contents: EmailContents,
}

impl Email {
    pub fn new(to: String, contents: EmailContents) -> Self {
        Self {
            to,
            contents,
            ..Default::default()
        }
    }

    /// Set the address of the email's sender.
    pub fn set_from(&mut self, from: Option<String>) {
        self.from = from;
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct EmailContents {
    /// Subject to use for the email.
    pub subject: String,
    /// HTML version of the email.
    pub html: String,
    /// Optional plain text version of the email.
    pub text: Option<String>,
}
