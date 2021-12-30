use crate::{email::template::RegisteredEmailTemplate, sms::template::RegisteredSmsTemplate};

#[derive(Default)]
pub struct Composite {
    email: Option<RegisteredEmailTemplate>,
    sms: Option<RegisteredSmsTemplate>,
}

impl Composite {
    /// Set the composite's email.
    pub fn set_email(&mut self, email: Option<RegisteredEmailTemplate>) {
        self.email = email;
    }

    /// Get a reference to the composite's email.
    pub fn email(&self) -> Option<&RegisteredEmailTemplate> {
        self.email.as_ref()
    }

    /// Set the composite's sms.
    pub fn set_sms(&mut self, sms: Option<RegisteredSmsTemplate>) {
        self.sms = sms;
    }

    /// Get a reference to the composite's sms.
    pub fn sms(&self) -> Option<&RegisteredSmsTemplate> {
        self.sms.as_ref()
    }
}
