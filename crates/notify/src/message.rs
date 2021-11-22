use crate::channel::ChannelType;

pub mod email;

pub enum Message {
    Email(email::Email),
}

impl Message {
    pub fn as_email(&self) -> Option<&email::Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn into_email(self) -> Option<email::Email> {
        match self {
            Self::Email(email) => Some(email),
        }
    }

    pub fn channel(&self) -> &'static ChannelType {
        match self {
            Self::Email(_) => &ChannelType::Email,
        }
    }
}
