use std::ops::Deref;

use notifier::Provider;

use crate::EmailMessage;

#[cfg(feature = "smtp")]
pub mod smtp;

pub mod test;

pub struct EmailProvider(Box<dyn Provider<Message = EmailMessage>>);

impl EmailProvider {
    pub fn new<T: Provider<Message = EmailMessage>>(inner: T) -> Self {
        Self(Box::new(inner))
    }
}

impl Deref for EmailProvider {
    type Target = Box<dyn Provider<Message = EmailMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
