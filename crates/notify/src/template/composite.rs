use crate::email::template::RegisteredEmailTemplate;

#[derive(Default)]
pub struct Composite {
    email: Option<RegisteredEmailTemplate>,
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
}

