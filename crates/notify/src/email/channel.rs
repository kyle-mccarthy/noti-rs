use super::{Address, EmailProvider};

pub struct EmailChannel {
    default_sender: Option<Address>,
    provider: Box<dyn EmailProvider>,
}

impl EmailChannel {
    pub fn new<T: EmailProvider + 'static>(provider: T) -> Self {
        Self {
            default_sender: None,
            provider: Box::new(provider),
        }
    }

    pub fn set_default_sender(&mut self, sender: Address) {
        self.default_sender = Some(sender);
    }
}
