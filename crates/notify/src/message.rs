use self::email::Email;
use crate::channel::ChannelType;

pub mod email;

pub enum Message {
    Email(email::Email),
}

impl Message {
    pub fn channel_type(&self) -> ChannelType {
        match self {
            Self::Email(_) => ChannelType::Email,
        }
    }

    pub fn is_email(&self) -> bool {
        matches!(self, Self::Email(_))
    }

    pub fn as_email(&self) -> Option<&Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn into_email(self) -> Option<Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }
}
