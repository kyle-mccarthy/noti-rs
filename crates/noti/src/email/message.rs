use serde::{Deserialize, Serialize};

use crate::channel::{ChannelType, Error};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    name: Option<String>,
    email: String,
}

impl Address {
    pub fn new(email: String, name: Option<String>) -> Self {
        Self { email, name }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    to: Address,
    from: Address,
    reply_to: Option<Address>,

    subject: String,
    html: String,
    text: Option<String>,
}

impl Email {
    pub fn to(&self) -> &Address {
        &self.to
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn reply_to(&self) -> &Option<Address> {
        &self.reply_to
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

#[derive(Debug, Default)]
pub struct EmailBuilder {
    pub(super) to: Option<Address>,
    pub(super) from: Option<Address>,
    pub(super) reply_to: Option<Address>,

    pub(super) subject: Option<String>,
    pub(super) html: Option<String>,
    pub(super) text: Option<String>,
}

impl EmailBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(mut self, email: String, name: Option<String>) -> Self {
        self.to = Some(Address::new(email, name));
        self
    }

    pub fn to_address(mut self, address: Address) -> Self {
        self.to = Some(address);
        self
    }

    pub fn from(mut self, email: String, name: Option<String>) -> Self {
        self.from = Some(Address::new(email, name));
        self
    }

    pub fn from_address(mut self, from: Address) -> Self {
        self.from = Some(from);
        self
    }

    pub fn reply_to(mut self, email: String, name: Option<String>) -> Self {
        self.reply_to = Some(Address::new(email, name));
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn html(mut self, html: String) -> Self {
        self.html = Some(html);
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn build(self) -> Result<Email, Error> {
        // required fields
        let to = self.to.ok_or(Error::MissingField {
            name: "to",
            channel_type: ChannelType::Email,
            context: None,
        })?;

        let from = self.from.ok_or(Error::MissingField {
            name: "from",
            channel_type: ChannelType::Email,
            context: None,
        })?;

        let subject = self.subject.ok_or(Error::MissingField {
            name: "subject",
            channel_type: ChannelType::Email,
            context: None,
        })?;

        let html = self.html.ok_or(Error::MissingField {
            name: "html",
            channel_type: ChannelType::Email,
            context: None,
        })?;

        // optional fields
        let text = self.text;
        let reply_to = self.reply_to;

        Ok(Email {
            to,
            from,
            reply_to,

            subject,
            html,
            text,
        })
    }
}
