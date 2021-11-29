use self::email::Email;
use crate::channel::ChannelType;

pub mod email;

pub enum Message {
    Email(email::Email),
}

impl Message {
    /// Gets the ChannelType of the message
    pub fn channel_type(&self) -> ChannelType {
        match self {
            Self::Email(_) => ChannelType::Email,
        }
    }

    /// Returns true if the message is an Email
    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email(_))
    }

    /// Attempts to return a reference to the Email. None is returned if the
    /// channel type doesn't match.
    pub fn as_email(&self) -> Option<&Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    /// Attempts to convert the Message into an Email. None is returned if the
    /// channel type doesn't match.
    pub fn into_email(self) -> Option<Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }
}
