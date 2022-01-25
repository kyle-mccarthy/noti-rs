use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress {
    name: Option<String>,
    email: String,
}

impl EmailAddress {
    pub fn new<E: ToString>(email: E, name: Option<String>) -> Self {
        Self {
            email: email.to_string(),
            name,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }
}
