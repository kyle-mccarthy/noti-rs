pub mod email;

pub trait Channel {
    type Message;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelType {
    Email,
    Sms,
    Push,
}
