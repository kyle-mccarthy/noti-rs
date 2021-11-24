use std::sync::Arc;

use super::Channel;
use crate::message::email::Email;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct EmailChannel(Arc<dyn EmailProvider>);

pub trait EmailProvider {
    fn send(&self, message: Email) -> Result<(), Error>;
}

impl Channel for EmailChannel {
    type Message = Email;
}
