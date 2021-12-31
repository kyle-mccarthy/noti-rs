pub mod channel;
pub mod message;
pub mod provider;
pub mod template;

pub use channel::EmailChannel;
pub use message::{Address, Email, EmailBuilder};
pub use template::EmailTemplate;
