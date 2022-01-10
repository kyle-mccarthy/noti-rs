use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress {
    name: Option<String>,
    email: String,
}

impl EmailAddress {
    pub fn new(email: String, name: Option<String>) -> Self {
        Self { email, name }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }
}
