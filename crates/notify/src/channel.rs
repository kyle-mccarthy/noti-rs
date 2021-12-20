use std::fmt::Display;

use fixed_map::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Key)]
pub enum ChannelType {
    Email,
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => f.write_str("Email"),
        }
    }
}
