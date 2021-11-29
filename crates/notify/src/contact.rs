#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Contact {
    Person(PersonContact),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContactType {
    Person,
}

impl ContactType {
    pub fn is_email(&self) -> bool {
        match self {
            Self::Person => true,
        }
    }
}

impl Contact {
    pub fn has_email(&self) -> bool {
        match self {
            Self::Person(_) => true,
        }
    }

    pub fn email(&self) -> Option<&String> {
        match self {
            Self::Person(PersonContact { email, .. }) => Some(email),
        }
    }

    pub fn contact_type(&self) -> ContactType {
        match self {
            Self::Person(_) => ContactType::Person,
        }
    }

    pub fn is_email(&self) -> bool {
        match self {
            Self::Person(_) => true,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonContact {
    name: Option<String>,
    email: String,
}

impl PersonContact {
    pub fn new(email: String, name: Option<String>) -> Self {
        Self { email, name }
    }

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

    pub fn into_contact(self) -> Contact {
        Contact::Person(self)
    }
}
