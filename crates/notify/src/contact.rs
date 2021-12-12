use crate::id::Id;

pub mod repository;

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct ContactId(pub Id);

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Contact {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl Contact {
    pub fn has_email(&self) -> bool {
        self.email.is_some()
    }

    /// Get a reference to the email contact's email.
    pub fn email(&self) -> Option<&String> {
        self.email.as_ref()
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
        self.email = Some(email);
    }
}

// pub struct EmailAddress {
//     pub name: String,
//     pub address: String,
// }

// pub struct PhoneNumber(String);

// pub struct DeviceId(String);

// pub trait Notifiable {
//     fn email(&self) -> Option<EmailAddress> {
//         None
//     }

//     fn phone_number(&self) -> Option<PhoneNumber> {
//         None
//     }

//     fn device_id(&self) -> Option<DeviceId> {
//         None
//     }
// }
