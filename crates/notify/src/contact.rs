#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contact {
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}

impl Contact {
    pub fn with_email<T: ToString>(email: T) -> Self {
        Self {
            email: Some(email.to_string()),
            ..Default::default()
        }
    }

    pub fn with_phone<T: ToString>(phone: T) -> Self {
        Self {
            phone: Some(phone.to_string()),
            ..Default::default()
        }
    }

    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    pub fn has_phone(&self) -> bool {
        self.phone.is_some()
    }

    /// Set the contact's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Get a reference to the contact's email.
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    /// Get a reference to the contact's phone.
    pub fn phone(&self) -> Option<&String> {
        self.phone.as_ref()
    }

    /// Get a reference to the contact's name.
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
