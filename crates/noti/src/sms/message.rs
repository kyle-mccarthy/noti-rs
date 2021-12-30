use crate::channel::{ChannelType, Error};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone_number: String) -> Self {
        Self(phone_number)
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct Sms {
    to: PhoneNumber,
    from: PhoneNumber,
    contents: String,
}

impl Sms {
    /// Get a reference to the sms's to.
    pub fn to(&self) -> &str {
        self.to.as_ref()
    }

    /// Get a reference to the sms's from.
    pub fn from(&self) -> &str {
        self.from.as_ref()
    }

    /// Get a reference to the sms's contents.
    pub fn contents(&self) -> &str {
        self.contents.as_ref()
    }
}

#[derive(Default, Debug)]
pub struct SmsBuilder {
    pub(crate) to: Option<PhoneNumber>,
    pub(crate) from: Option<PhoneNumber>,
    pub(crate) contents: Option<String>,
}

impl SmsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Sms, Error> {
        let to = self.to.ok_or(Error::MissingField {
            name: "to",
            channel_type: ChannelType::Sms,
            context: None,
        })?;

        let from = self.from.ok_or(Error::MissingField {
            name: "from",
            channel_type: ChannelType::Sms,
            context: None,
        })?;

        let contents = self.contents.ok_or(Error::MissingField {
            name: "contents",
            channel_type: ChannelType::Sms,
            context: None,
        })?;

        Ok(Sms { to, from, contents })
    }

    /// Set the sms builder's to.
    pub fn to(mut self, to: PhoneNumber) -> Self {
        self.to = Some(to);
        self
    }

    /// Set the sms builder's from.
    pub fn from(mut self, from: PhoneNumber) -> Self {
        self.from = Some(from);
        self
    }

    /// Set the sms builder's contents.
    pub fn contents(mut self, contents: String) -> Self {
        self.contents = Some(contents);
        self
    }
}
