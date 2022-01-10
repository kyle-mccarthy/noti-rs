use serde::{Deserialize, Serialize};

use crate::EmailAddress;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailContents {
    subject: String,
    html: String,
    text: Option<String>,
}

impl EmailContents {
    pub fn new(subject: String, html: String, text: Option<String>) -> Self {
        Self {
            subject,
            html,
            text,
        }
    }

    /// Get a reference to the email contents's subject.
    pub fn subject(&self) -> &str {
        self.subject.as_ref()
    }

    /// Set the email contents's subject.
    pub fn set_subject(&mut self, subject: String) {
        self.subject = subject;
    }

    /// Get a reference to the email contents's html.
    pub fn html(&self) -> &str {
        self.html.as_ref()
    }

    /// Set the email contents's html.
    pub fn set_html(&mut self, html: String) {
        self.html = html;
    }

    /// Get a reference to the email contents's text.
    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    /// Set the email contents's text.
    pub fn set_text(&mut self, text: Option<String>) {
        self.text = text;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailMessage {
    to: EmailAddress,
    from: EmailAddress,
    reply_to: Option<EmailAddress>,

    contents: EmailContents,
}

impl EmailMessage {
    pub fn new(to: EmailAddress, from: EmailAddress, contents: EmailContents) -> Self {
        Self {
            to,
            from,
            reply_to: None,
            contents,
        }
    }

    /// Get a reference to the email message's to.
    pub fn to(&self) -> &EmailAddress {
        &self.to
    }

    /// Set the email message's to.
    pub fn set_to(&mut self, to: EmailAddress) {
        self.to = to;
    }

    /// Get a reference to the email message's from.
    pub fn from(&self) -> &EmailAddress {
        &self.from
    }

    /// Set the email message's from.
    pub fn set_from(&mut self, from: EmailAddress) {
        self.from = from;
    }

    /// Get a reference to the email message's reply to.
    pub fn reply_to(&self) -> Option<&EmailAddress> {
        self.reply_to.as_ref()
    }

    /// Set the email message's reply to.
    pub fn set_reply_to(&mut self, reply_to: Option<EmailAddress>) {
        self.reply_to = reply_to;
    }

    /// Get a reference to the email message's contents.
    pub fn contents(&self) -> &EmailContents {
        &self.contents
    }

    /// Set the email message's contents.
    pub fn set_contents(&mut self, contents: EmailContents) {
        self.contents = contents;
    }
}
