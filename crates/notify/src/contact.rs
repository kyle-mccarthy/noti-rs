#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Contact {
    Email(EmailContact),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContactType {
    Email,
}

impl ContactType {
    pub fn is_email(&self) -> bool {
        match self {
            Self::Email => true,
        }
    }
}

impl Contact {
    pub fn has_email(&self) -> bool {
        match self {
            Self::Email(_) => true,
        }
    }

    pub fn email(&self) -> Option<&String> {
        match self {
            Self::Email(EmailContact { email, .. }) => Some(email),
        }
    }

    pub fn contact_type(&self) -> ContactType {
        match self {
            Self::Email(_) => ContactType::Email,
        }
    }

    pub fn is_email(&self) -> bool {
        match self {
            Self::Email(_) => true,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailContact {
    name: Option<String>,
    email: String,
}

impl EmailContact {
    // const CONTACT_TYPE: ContactType = ContactType::Email;

    /// Set the email contact's name.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Get a reference to the email contact's name.
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Set the email contact's email.
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    /// Get a reference to the email contact's email.
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }
}
