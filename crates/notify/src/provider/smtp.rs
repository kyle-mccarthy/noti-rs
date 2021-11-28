use async_trait::async_trait;

use super::{DynProvider, Error, Provider};
use crate::{channel::EmailChannel, message::email::Email};

pub struct SmtpProvider {}

#[async_trait]
impl Provider for SmtpProvider {
    type Channel = EmailChannel;

    async fn send(&self, _message: Email) -> Result<(), Error> {
        todo!()
    }

    fn id(&self) -> &str {
        "smtp-provider"
    }

    fn into_dyn_provider(self) -> DynProvider {
        DynProvider::Email(Box::new(self))
    }
}
